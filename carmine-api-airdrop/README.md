# AirDrop

This component creates a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) from an array of `address` and `amount`. The hashing function used is `starknet_crypto::pedersen_hash`.

Tree consists of `Nodes` and a node that has no child node is considered a `leaf`.

Tree is built recursivelly, first set of nodes (leaves) are created from each `address`, `amount` pair as `pedersen_hash(address, amount)` - note the order of hashed values, it is importatnt. Then we keep hashing together pairs of node hashes until there is just one - that is the root of the Merkle tree. If there is an odd number of nodes other than 1 (if it is 1 we have found the root), we clone the last node, that way the last nodes is hashed with itself.

## Validation

To check that given `address` and `amount` is part of the Merkle tree we need to know which nodes it was paired with. We go from the root, following the path that contains given `address` and collect all the hashesof the other nodes along the way. The list is then reversed to be leaf first, root last.

Smart Contract can then take `address`, `amount` and proof array and hash them all together to generate root, this root is then compared with the correct root stored in the SC memory.

## Usage

1. update `air-drop.json` to contain correct addresses and amounts
2. get the root by running
```sh
cargo run -p carmine-api-airdrop --bin get_root
```
3. get the proof for any address by calling
```rust
let proof: Vec<String> = merkle_tree.address_calldata(address);
```
4. call SmartContract with `[address, amount, ...proof]`
