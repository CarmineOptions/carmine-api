use carmine_api_airdrop::merkle_tree::MerkleTree;

fn main() {
    let root = MerkleTree::new().root.value;
    let base16 = format!("{:#x}", root);
    println!("Merkle Tree root: {}\nHex format: {}", root, base16);
}
