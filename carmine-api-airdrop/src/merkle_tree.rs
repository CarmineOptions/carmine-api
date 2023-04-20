use serde::Deserialize;
use starknet_crypto::{pedersen_hash, FieldElement};
use std::fs::File;

#[derive(Deserialize, Debug, Clone)]
pub struct Airdrop {
    pub address: String,
    amount: u32,
}

#[allow(dead_code)]
pub struct MerkleTree {
    pub root: String,
    tree: Vec<Vec<Node>>,
    leaves: Vec<Airdrop>,
}

impl MerkleTree {
    pub fn new() -> Self {
        let mut leaves = read_airdrop();

        if leaves.len() % 2 == 1 {
            // can safely unwrap, because length is odd
            leaves.push(leaves.last().unwrap().clone());
        }

        // hashed leave is hash(address, amount_b16)
        let hashed_leaves: Vec<Node> = leaves
            .iter()
            .map(|v| Node::new(v.address.clone(), format!("{:#x}", v.amount)))
            .collect();

        let (tree, root) = match build_tree_recursively(TreeBuilder::KeepGoing(vec![hashed_leaves]))
        {
            TreeBuilder::Done((tree, root)) => (tree, root),
            _ => unreachable!("Unexpected build_tree_recursively result"),
        };

        MerkleTree { root, tree, leaves }
    }

    // Get array of strings that should be used as arguments for the SC
    pub fn address_calldata(&self, address: &str) -> Option<Vec<String>> {
        let airdrop_index_option = (
            self.leaves.iter().find(|v| v.address == address),
            self.leaves.iter().position(|v| v.address == address),
        );
        let (airdrop, mut index) = match airdrop_index_option {
            (Some(airdrop), Some(index)) => (airdrop, index),
            _ => {
                return None;
            }
        };

        let mut calldata: Vec<String> = vec![
            // address tokens should be sent to
            airdrop.address.to_string(),
            // base16 amount to send
            format!("{:#x}", airdrop.amount),
        ];

        // every node up to the root (excluded)
        for floor in &self.tree {
            calldata.push(floor[index_switch(index)].value.clone());
            index = index / 2;
        }

        Some(calldata)
    }
    #[allow(dead_code)]
    // for debugging
    pub fn print(&self) {
        let mut floor_count = 0;

        for floor in self.tree.iter() {
            print!("Floor {}:\n", floor_count);
            let mut it = floor.iter().peekable();
            while let Some(node) = it.next() {
                if it.peek().is_none() {
                    println!("{}", node.value);
                } else {
                    print!("{} - ", node.value);
                }
            }
            println!();
            floor_count = floor_count + 1;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
    value: String,
}

impl Node {
    fn new(left: String, right: String) -> Self {
        let value = hash(&left, &right);
        Node { left, right, value }
    }
}

enum TreeBuilder {
    KeepGoing(Vec<Vec<Node>>),
    Done((Vec<Vec<Node>>, String)),
}

fn index_switch(index: usize) -> usize {
    match index % 2 {
        0 => index + 1,
        1 => index - 1,
        _ => unreachable!("mod 2 of index must be 0 or 1"),
    }
}

fn build_tree_recursively(tree_builder: TreeBuilder) -> TreeBuilder {
    let (current_floor, tree) = match tree_builder {
        TreeBuilder::KeepGoing(tree) => (tree.last().unwrap().clone(), tree),
        TreeBuilder::Done((tree, root)) => return TreeBuilder::Done((tree, root)),
    };

    let mut next_floor = current_floor
        .chunks(2)
        .map(|pair| Node::new(pair[0].value.clone(), pair[1].value.clone()))
        .collect::<Vec<_>>();

    if current_floor.len() == 2 {
        return TreeBuilder::Done((tree, next_floor[0].value.clone()));
    }

    if next_floor.len() % 2 == 1 {
        // if odd - pair last element with itself
        next_floor.push(next_floor.last().unwrap().clone());
    }

    let mut new_tree = tree.to_vec();
    new_tree.push(next_floor);

    build_tree_recursively(TreeBuilder::KeepGoing(new_tree))
}

pub fn hash(a: &str, b: &str) -> String {
    let l = FieldElement::from_hex_be(a).unwrap();
    let r = FieldElement::from_hex_be(b).unwrap();

    if l.gt(&r) {
        return format!("{:#x}", pedersen_hash(&l, &r));
    }
    format!("{:#x}", pedersen_hash(&r, &l))
}

pub fn read_airdrop() -> Vec<Airdrop> {
    let file = File::open("./src/air-drop.json").expect("Failed to read file");
    let reader = std::io::BufReader::new(file);
    let airdrop: Vec<Airdrop> = serde_json::from_reader(reader).expect("Failed to parse airdrop");
    airdrop
}
