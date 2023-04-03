use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::VecDeque, error::Error};

trait IMerkleTree<K: Serialize, V: Serialize> {
    fn new() -> Self;

    // The Insert function takes a key and value as arguments. It will traverse
    // Merkle Tree, find the rightmost place to insert the entry. Entry is an object consisting of
    // ({key, value, etc..}). Merkle Tree consist of keys, where a key is hash of the JSON string
    // of the entry and value is the JSON string of the entry. Insert function will return a string
    // which will be the new Root hash. After every insert returned Root hash will correspond
    // to the latest state of the Merkle Tree.
    fn insert(&mut self, entry: MerkleEntry<K, V>) -> [u8; 32];

    // The Delete function takes a key (Entry) as argument, traverses the Merkle
    // Tree and finds that key. If the key exists, delete the corresponding Entry and re-balance
    // the tree if necessary. Delete function will return updated root hash if the key was found
    // otherwise return empty string (or ‘’path_not_found”) if the key doesn't exist.
    fn delete(&mut self, entry: MerkleEntry<K, V>);

    // The GenerateMerklePath function takes a key (Entry object) and return the
    // Merkle Path of this key in the form of the ordered list of hashes, starting from the leaf. If
    // the key does not exist, then return empty string (or ‘’path_not_found”).
    fn generate_path(&self, entry: MerkleEntry<K, V>);

    // The VerifyMerklePath function takes a key (Entry) and its Merkle path, the
    // ordered list of sibling hashes as argument. It computes all the hashes on the path from
    // the given Entry to the root using the location and the MerklePath. The newly computed
    // root hash is compared to the stored root for verification. Function returns true if the
    // verification succeeds (if the newly computed root hash is equal to the stored root hash)
    // otherwise return false.
    fn verify_path(&self, key: K, location: String, path: Vec<MerkleNode<K, V>>) -> bool;
}

#[derive(Clone, Serialize, Deserialize)]
struct MerkleTree<K: Serialize, V: Serialize> {
    root_hash: [u8; 32],
    root_node: Option<MerkleNode<K, V>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct MerkleNode<K: Serialize, V: Serialize> {
    value: [u8; 32],
    left: Option<Box<MerkleNode<K, V>>>,
    right: Option<Box<MerkleNode<K, V>>>,
    leaf_value: MerkleEntry<K, V>,
}

#[derive(Clone, Serialize, Deserialize)]
struct MerkleEntry<K: Serialize, V: Serialize> {
    key: K,
    value: V,
}

impl<K, V> IMerkleTree<K, V> for MerkleTree<K, V>
where
    K: Serialize + PartialEq + Eq + Clone,
    V: Serialize + Clone,
{
    fn new() -> Self {
        return MerkleTree {
            root_hash: [0; 32],
            root_node: None,
        };
    }

    fn insert(&mut self, entry: MerkleEntry<K, V>) -> [u8; 32] {
        // Calculate the hash of the entry
        let hash = calculate_hash(&entry).unwrap();
        let mut new_root_hash = [0; 32];
        let mut new_root_node = None;

        // If the root node is None, the tree is empty, so create a new root node with the entry as leaf
        if self.root_node.is_none() {
            self.root_node = Some(MerkleNode {
                value: hash,
                left: None,
                right: None,
                leaf_value: entry,
            });
            self.root_hash = hash;
            return self.root_hash;
        }

        // Create a stack to traverse the tree from right to left
        let mut stack: Vec<(Option<&MerkleNode<K, V>>, Option<Box<MerkleNode<K, V>>>)> = Vec::new();
        stack.push((Some(&self.root_node.as_ref().unwrap()), None));

        // Traverse the tree from right to left, to find the rightmost place to insert the entry
        while let Some((node, parent)) = stack.pop() {
            if let Some(node) = node {
                // If the current node is a leaf and has the same key as the entry,
                // update its value and return the root hash
                if node.left.is_none() && node.right.is_none() && node.leaf_value.key == entry.key {
                    node.leaf_value = entry;
                    new_root_hash = self.root_hash;
                    new_root_node = self.root_node.take();
                    break;
                }
                // If the current node is a leaf and has a different key than the entry,
                // create new leaf nodes for the entry and the current leaf node, and insert them
                // as children of a new parent node, which is then attached to the parent of the current node
                else if node.left.is_none() && node.right.is_none() {
                    let new_leaf_node = MerkleNode {
                        value: hash,
                        left: None,
                        right: None,
                        leaf_value: entry,
                    };
                    let current_leaf_node = node;
                    let new_parent_node = MerkleNode {
                        value: calculate_parent_hash(
                            &current_leaf_node.value,
                            &new_leaf_node.value,
                        ),
                        left: Some(Box::new(current_leaf_node)),
                        right: Some(Box::new(new_leaf_node)),
                        leaf_value: MerkleEntry {
                            key: current_leaf_node.leaf_value.key.clone(),
                            value: serde_json::to_value(&current_leaf_node.leaf_value)
                                .unwrap()
                                .to_string(),
                        },
                    };
                    if let Some(parent_node) = parent {
                        if parent_node.left.as_ref().unwrap().value == current_leaf_node.value {
                            parent_node.left = Some(Box::new(new_parent_node));
                        } else {
                            parent_node.right = Some(Box::new(new_parent_node));
                        }
                    } else {
                        new_root_node = Some(new_parent_node);
                        new_root_hash = new_parent_node.value;
                    }
                    break;
                }
                // If the current node is not a leaf, add its right and left children to the stack
                else {
                    stack.push((node.left.as_ref(), Some(Box::new(node.clone()))));
                    stack.push((node.right.as_ref(), Some(Box::new(node.clone()))));
                }
            }
        }

        // If a new root node was created, update the root hash and root node
        if let Some(new_node) = new_root_node {
            self.root_node = Some(new_root_node);
            self.root_hash = new_root_hash;
        }
        self.root_hash
        // Return the root hash
    }

    // fn insert(&mut self, entry: MerkleEntry<K, V>) -> [u8; 32] {
    //     let hash: [u8; 32] = calculate_hash(&entry).unwrap();

    //     // Handle base case where root node is None
    //     if self.root_node.is_none() {
    //         self.root_node = Some(MerkleNode {
    //             value: hash,
    //             left: None,
    //             right: None,
    //             leaf_value: entry.clone(),
    //         });
    //         self.root_hash = hash;
    //         return self.root_hash;
    //     }

    //     // VecDequeue offers push_back and pop_front methods
    //     let mut queue: VecDeque<(Option<&MerkleNode<K, V>>, Option<Box<MerkleNode<K, V>>>)> =
    //         VecDeque::new();
    //     queue.push_back((Some(&self.root_node.as_ref().unwrap()), None));

    //     // tree traversal
    //     let mut mutable_node = &mut self.root_node;
    //     while let Some((node, parent)) = queue.pop_front() {}

    //     return [0; 32];
    // }

    fn delete(&mut self, entry: MerkleEntry<K, V>) {}

    fn generate_path(&self, entry: MerkleEntry<K, V>) {}

    fn verify_path(&self, key: K, location: String, path: Vec<MerkleNode<K, V>>) -> bool {
        return true;
    }
}

fn calculate_hash<T: Serialize>(data: &T) -> Result<[u8; 32], Box<dyn Error>> {
    let json = serde_json::to_string(data)?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let result = hasher.finalize();
    let mut hash = [0; 32];
    hash.copy_from_slice(result.as_slice());
    Ok(hash)
}

fn calculate_parent_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();
    println!("{:?}", result);

    let merkle_tree: MerkleTree<String, String> = MerkleTree::new();
}
