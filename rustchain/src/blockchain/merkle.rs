// use serde::{Deserialize, Serialize};
// use sha2::{Digest, Sha256};

// trait IMerkleTree {
//     // The Insert function takes a key and value as arguments. It will traverse
//     // Merkle Tree, find the rightmost place to insert the entry. Entry is an object consisting of
//     // ({key, value, etc..}). Merkle Tree consist of keys, where a key is hash of the JSON string
//     // of the entry and value is the JSON string of the entry. Insert function will return a string
//     // which will be the new Root hash. After every insert returned Root hash will correspond
//     // to the latest state of the Merkle Tree.
//     fn insert(&mut self, entry: MerkleEntry) -> Result<[u8; 32], String>;

//     // The Delete function takes a key (Entry) as argument, traverses the Merkle
//     // Tree and finds that key. If the key exists, delete the corresponding Entry and re-balance
//     // the tree if necessary. Delete function will return updated root hash if the key was found
//     // otherwise return empty string (or ‘’path_not_found”) if the key doesn't exist.
//     fn delete(&mut self, entry: &MerkleEntry) -> Result<[u8; 32], String>;

//     // The GenerateMerklePath function takes a key (Entry object) and return the
//     // Merkle Path of this key in the form of the ordered list of hashes, starting from the leaf. If
//     // the key does not exist, then return empty string (or ‘’path_not_found”).
//     fn generate_merkle_path(&self, entry: MerkleEntry) -> Result<Vec<MerkleNode>, String>;

//     // The VerifyMerklePath function takes a key (Entry) and its Merkle path, the
//     // ordered list of sibling hashes as argument. It computes all the hashes on the path from
//     // the given Entry to the root using the location and the MerklePath. The newly computed
//     // root hash is compared to the stored root for verification. Function returns true if the
//     // verification succeeds (if the newly computed root hash is equal to the stored root hash)
//     // otherwise return false.
//     fn verify_path(&self, key: [u8; 32], location: String, path: Vec<MerkleNode>) -> bool;
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct MerkleTree {
//     pub root_hash: [u8; 32],
//     pub root_node: Option<Box<MerkleNode>>,
//     pub depth: i32,
//     pub leaves: Vec<Box<MerkleNode>>,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct MerkleNode {
//     pub value: [u8; 32],
//     pub left: Option<Box<MerkleNode>>,
//     pub right: Option<Box<MerkleNode>>,
//     pub leaf: Option<MerkleEntry>,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct MerkleEntry {
//     pub key: [u8; 32],
//     pub value: Vec<u8>,
// }

// impl IMerkleTree for MerkleTree {
//     fn insert(&mut self, entry: MerkleEntry) -> Result<[u8; 32], String> {
//         let leaf = self.new_leaf(entry.clone());

//         // empty tree
//         if self.root_node.is_none() {
//             self.root_node = Some(leaf);
//             self.root_hash = leaf.value;
//             self.depth = 1;
//             self.leaves.push(leaf);
//             return Ok(self.root_hash);
//         }

//         let num_leaves: f64 = self.leaves.len() as f64;
//         let max_leaves: f64 = (2f64).powf(self.depth as f64 - 1.0);

//         // merkle tree is full - add new root
//         if num_leaves == max_leaves {
//             let new_root = self.new_node(self.root_node.take(), Some(leaf.clone()));
//             self.root_node = Some(new_root.clone());
//             self.root_hash = new_root.value;
//             self.depth += 1;

//             let mut node = new_root;
//             for _ in 0..(self.depth - 1) {
//                 let temp_node = self.new_node(Some(leaf.clone()), None);
//                 node.left.as_mut().unwrap().right = Some(temp_node.clone());
//                 node = temp_node;
//             }
//             self.leaves.push(leaf);
//             return Ok(self.root_hash);
//         }

//         // adding new leaves
//         let mut parents = vec![];
//         let mut current_node = self.root_node.as_ref().unwrap();
//         while current_node.left.is_some() || current_node.right.is_some() {
//             parents.push(current_node);
//             if current_node.right.is_some() {
//                 current_node = current_node.right.as_ref().unwrap();
//             } else {
//                 current_node = current_node.left.as_ref().unwrap();
//             }
//         }

//         let is_even = self.leaves.len() % 2 == 0;
//         let mut parent = parents.pop().unwrap();
//         if is_even {
//             let mut counter = 0;
//             while parent.right.is_some() {
//                 parent = parents.pop().unwrap();
//                 counter += 1;
//             }

//             let mut current = parent;
//             for _ in 0..counter {
//                 let new_node = self.new_node(Some(leaf.clone()), None);
//                 if current.right.is_none() {
//                     current.right = Some(new_node.clone());
//                 } else {
//                     current.left = Some(new_node.clone());
//                 }
//                 current = &new_node;
//             }
//             current.left = Some(leaf.clone());
//         } else {
//             parent.right = Some(leaf.clone());
//             parent.value = self.digest_nodes(parent.left.as_ref().unwrap().value, leaf.value);
//         }
//         let mut current_parent = parent;
//         while let Some(parent) = parents.pop() {
//             let left_hash = parent.left.as_ref().unwrap().value;
//             let right_hash = parent.right.as_ref().unwrap().value;
//             parent.value = self.digest_nodes(left_hash, right_hash);
//             current_parent = parent;
//         }

//         self.root_hash = current_parent.value;
//         self.leaves.push(leaf);
//         Ok(self.root_hash)
//     }

//     fn delete(&mut self, entry: &MerkleEntry) -> Result<[u8; 32], String> {
//         // only 1 node
//         if self.leaves.len() == 1 {
//             self.root_node = None;
//             self.root_hash = [0; 32];
//             self.depth = 0;
//             self.leaves = Vec::new();
//             return Ok(self.root_hash);
//         }

//         // case 2 nodes
//         if self.leaves.len() == 2 {
//             if self.entries_equal(
//                 entry,
//                 &self
//                     .root_node
//                     .as_ref()
//                     .unwrap()
//                     .left
//                     .as_ref()
//                     .unwrap()
//                     .leaf
//                     .unwrap(),
//             ) {
//                 self.root_node = self.root_node.as_ref().unwrap().right.clone();
//                 self.root_hash = self.root_node.as_ref().unwrap().value;
//                 self.depth = 1;
//                 self.leaves = vec![self.root_node.as_ref().unwrap().clone()];
//                 return Ok(self.root_hash);
//             }
//             self.root_node = self.root_node.as_ref().unwrap().left.clone();
//             self.root_hash = self.root_node.as_ref().unwrap().value;
//             self.depth = 1;
//             self.leaves = vec![self.root_node.as_ref().unwrap().clone()];
//             return Ok(self.root_hash);
//         }

//         // deleting the only node on the right side
//         if ((2f64).powi(self.depth - 1) / 2.0).floor() as usize + 1 == self.leaves.len()
//             && self.digest_entry(entry)
//                 == self
//                     .root_node
//                     .as_ref()
//                     .unwrap()
//                     .right
//                     .as_ref()
//                     .unwrap()
//                     .value
//             && self.leaves.len() > 2
//         {
//             self.root_node = self.root_node.as_ref().unwrap().left.clone();
//             self.root_hash = self
//                 .root_node
//                 .as_ref()
//                 .unwrap()
//                 .left
//                 .as_ref()
//                 .unwrap()
//                 .value;
//             self.depth -= 1;
//             for (i, leaf) in self.leaves.iter().enumerate() {
//                 if self.entries_equal(leaf.leaf.as_ref().unwrap(), entry) {
//                     self.leaves.remove(i);
//                     return Ok(self.root_hash);
//                 }
//             }
//         }

//         let mut leaves = Vec::new();
//         let mut pending_leaves = Vec::new();
//         let mut is_on_left_side = false;
//         for (i, leaf) in self.leaves.iter().enumerate() {
//             if self.entries_equal(leaf.leaf.as_ref().unwrap(), entry) {
//                 leaves = self.leaves[..i].to_vec();
//                 pending_leaves = self.leaves[i + 1..].to_vec();
//                 if i < ((2f64).powi(self.depth - 1) / 2.0).floor() as usize {
//                     is_on_left_side = true;
//                 }
//                 break;
//             }
//         }

//         // let (path, found) = self.find_leaf(self.root_node.as_ref().unwrap(), entry);
//         // if !found {
//         //     return Err(String::from("Something went wrong searching entry"));
//         // }
//         // let location = path.clone().unwrap();
//         // let target = location.last().unwrap();
//         // for i in (0..location.len() - 1).rev() {
//         //     let loc = location[i];
//         //     let left = *loc.left.clone().unwrap();
//         //     if left == **target {
//         //         loc.left = loc.right.clone();
//         //         loc.right = None;
//         //     } else {
//         //         loc.right = loc.left.clone();
//         //         loc.left = None;
//         //     }
//         // }
//         let (path, found) = self.find_leaf(self.root_node.as_ref().unwrap(), entry);
//         if !found {
//             return Err(String::from("Entry not found"));
//         }
//         let location = path.unwrap();
//         let target = location.last().unwrap();
//         for i in (0..location.len() - 1).rev() {
//             let mut loc: MerkleNode = location[i].clone();
//             let left = loc.left.as_ref().unwrap().clone();
//             if left.value.as_ptr() == target.value.as_ptr() {
//                 loc.left = loc.right.take();
//                 loc.value = loc.left.as_ref().unwrap().value;
//             } else {
//                 loc.right = loc.left.take();
//                 loc.value = loc.right.as_ref().unwrap().value;
//             }
//         }

//         if is_on_left_side {
//             self.root_node = self.root_node.as_ref().unwrap().left.clone();
//             self.root_hash = self
//                 .root_node
//                 .as_ref()
//                 .unwrap()
//                 .left
//                 .as_ref()
//                 .unwrap()
//                 .value;
//             self.depth -= 1;
//             self.leaves = leaves;

//             for node in pending_leaves {
//                 self.insert(node.leaf.unwrap())?;
//             }
//         } else {
//             self.root_node = self.root_node.as_ref().unwrap().right.clone();
//             self.root_hash = self
//                 .root_node
//                 .as_ref()
//                 .unwrap()
//                 .right
//                 .as_ref()
//                 .unwrap()
//                 .value;
//             self.depth -= 1;
//             self.leaves = leaves;

//             for node in pending_leaves {
//                 self.insert(node.leaf.unwrap())?;
//             }
//         }

//         Ok(self.root_hash)
//     }

//     // fn generate_merkle_path(&self, entry: MerkleEntry) -> Result<Vec<MerkleNode>, String> {
//     //     Ok(vec![])
//     // }

//     fn generate_merkle_path(&self, entry: MerkleEntry) -> Result<Vec<MerkleNode>, String> {
//         let (path, found) = self.find_leaf(&self.root_node.as_ref().unwrap(), &entry);
//         if !found {
//             return Err("Entry not found".to_string());
//         }

//         let locations = path.clone().unwrap();
//         let mut merkle_path = vec![];
//         let mut visited_node: &MerkleNode = locations[locations.len() - 1];
//         for i in (0..locations.len() - 1).rev() {
//             let node = locations[i];

//             if node == visited_node || node.right.is_none() {
//                 visited_node = node;
//                 continue; // skip intermediate nodes
//             }

//             let right = *node.right.clone().unwrap();
//             if right == *visited_node {
//                 merkle_path.push(*node.left.as_ref().unwrap().clone());
//             } else {
//                 merkle_path.push(*node.right.as_ref().unwrap().clone());
//             }
//             visited_node = node;
//         }

//         Ok(merkle_path)
//     }

//     fn verify_path(&self, key: [u8; 32], location: String, path: Vec<MerkleNode>) -> bool {
//         true
//     }
// }

// impl MerkleTree {
//     pub fn new() -> Self {
//         Self {
//             root_hash: [0; 32],
//             root_node: None,
//             depth: 0,
//             leaves: vec![],
//         }
//     }

//     fn new_node(
//         &self,
//         left: Option<Box<MerkleNode>>,
//         right: Option<Box<MerkleNode>>,
//     ) -> Box<MerkleNode> {
//         let left_hash = left.as_ref().unwrap().value;
//         let right_hash = if let Some(right_node) = right.as_ref() {
//             right_node.value
//         } else {
//             [0; 32]
//         };

//         let value = self.digest_nodes(left_hash, right_hash);
//         Box::new(MerkleNode {
//             value,
//             left,
//             right,
//             leaf: None,
//         })
//     }

//     fn new_leaf(&self, entry: MerkleEntry) -> Box<MerkleNode> {
//         let value = entry.key;
//         Box::new(MerkleNode {
//             value,
//             left: None,
//             right: None,
//             leaf: Some(entry),
//         })
//     }

//     fn digest_nodes(&self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
//         let mut hasher = Sha256::new();
//         hasher.update(left);
//         hasher.update(right);
//         let hash = hasher.finalize();
//         let mut value = [0; 32];
//         value.copy_from_slice(&hash[..]);
//         value
//     }

//     fn digest_entry(&self, entry: &MerkleEntry) -> [u8; 32] {
//         let serialized = serde_json::to_vec(entry).unwrap();
//         let mut hasher = Sha256::new();
//         hasher.update(serialized);
//         let result = hasher.finalize();
//         let mut hash = [0u8; 32];
//         hash.copy_from_slice(&result[..]);
//         hash
//     }

//     fn entries_equal(&self, entry1: &MerkleEntry, entry2: &MerkleEntry) -> bool {
//         self.digest_entry(entry1) == self.digest_entry(entry2)
//     }

//     fn find_leaf<'a>(
//         &'a self,
//         node: &'a MerkleNode,
//         entry: &'a MerkleEntry,
//     ) -> (Option<Vec<&'a MerkleNode>>, bool) {
//         if self.entries_equal(node.leaf.as_ref().unwrap(), entry) {
//             return (Some(vec![node]), true);
//         }
//         let (left_path, found) = self.find_leaf(node.left.as_ref().unwrap(), entry);
//         if found {
//             return (Some([vec![node], left_path.unwrap()].concat()), true);
//         }
//         let (right_path, found) = self.find_leaf(node.right.as_ref().unwrap(), entry);
//         if found {
//             return (Some([vec![node], right_path.unwrap()].concat()), true);
//         }
//         (None, false)
//     }
// }
