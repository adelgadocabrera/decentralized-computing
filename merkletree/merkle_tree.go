package merkletree

import (
	"crypto/sha256"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"strings"

	"github.com/gammazero/deque"
)

type IMerkleTree interface {
	// The Insert function takes a key and value as arguments. It will traverse
	// Merkle Tree, find the rightmost place to insert the entry. Entry is an object consisting of
	// ({key, value, etc..}). Merkle Tree consist of keys, where a key is hash of the JSON string
	// of the entry and value is the JSON string of the entry. Insert function will return a string
	// which will be the new Root hash. After every insert returned Root hash will correspond
	// to the latest state of the Merkle Tree.
	Insert(*MerkleEntry) ([32]byte, error)

	// The Delete function takes a key (Entry) as argument, traverses the Merkle
	// Tree and finds that key. If the key exists, delete the corresponding Entry and re-balance
	// the tree if necessary. Delete function will return updated root hash if the key was found
	// otherwise return empty string (or ‘’path_not_found”) if the key doesn't exist.
	Delete(*MerkleEntry) ([32]byte, error)

	// The GenerateMerklePath function takes a key (Entry object)and return the
	// Merkle Path of this key in the form of the ordered list of hashes, starting from the leaf. If
	// the key does not exist, then return empty string (or ‘’path_not_found”).
	GenerateMerklePath(*MerkleEntry) ([][32]byte, bool)

	// The VerifyMerklePath function takes a key (Entry) and its Merkle path, the
	// ordered list of sibling hashes as argument. It computes all the hashes on the path from
	// the given Entry to the root using the location and the MerklePath. The newly computed
	// root hash is compared to the stored root for verification. Function returns true if the
	// verification succeeds (if the newly computed root hash is equal to the stored root hash)
	// otherwise return false.
	VerifyMerklePath(*MerkleEntry, int, [][32]byte) bool
}

type MerkleTree struct {
	rootHash [32]byte
	rootNode *MerkleNode
	depth    int
	leaves   []*MerkleNode
}

type MerkleNode struct {
	value [32]byte
	left  *MerkleNode
	right *MerkleNode
	leaf  *MerkleEntry
}

type MerkleEntry struct {
	Key   [32]byte
	Value []byte
}

func NewMerkleTree() *MerkleTree {
	return &MerkleTree{
		rootHash: [32]byte{},
		rootNode: nil,
		depth:    0,
		leaves:   []*MerkleNode{},
	}
}

func (mt *MerkleTree) NewNode(left *MerkleNode, right *MerkleNode) *MerkleNode {
	if right == nil { // temp node
		return &MerkleNode{
			value: mt.digestEntry(left.leaf),
			left:  left,
			right: right,
			leaf:  nil,
		}
	}
	return &MerkleNode{
		value: mt.digestNodes(left.value, right.value),
		left:  left,
		right: right,
		leaf:  nil,
	}
}

func (mt *MerkleTree) NewLeaf(entry *MerkleEntry) *MerkleNode {
	return &MerkleNode{
		value: mt.digestEntry(entry),
		left:  nil,
		right: nil,
		leaf:  entry,
	}
}

func (mt *MerkleTree) Insert(entry *MerkleEntry) ([32]byte, error) {
	leaf := mt.NewLeaf(entry)

	// handle case for empty tree
	if mt.rootNode == nil {
		mt.rootNode = leaf
		mt.rootHash = leaf.value
		mt.depth = 1
		mt.leaves = append(mt.leaves, leaf)
		return mt.rootHash, nil
	}

	// Handle case for full binary tree:
	// Create new root node and old tree is now left subtree
	// and new leaf is rightmost node. Creates temp nodes to
	// match height
	if len(mt.leaves) == int(math.Pow(2, float64(mt.depth-1))) {
		newRoot := mt.NewNode(mt.rootNode, leaf)
		node := newRoot
		for i := 0; i < mt.depth-1; i++ {
			n := mt.NewNode(leaf, nil)
			if i == 0 {
				node.right = n
			} else {
				node.left = n
			}
			node = n
		}
		mt.rootNode = newRoot
		mt.rootHash = newRoot.value
		mt.depth = mt.depth + 1
		mt.leaves = append(mt.leaves, leaf)
		return mt.rootHash, nil
	}

	// get rightmost node
	currentNode := mt.rootNode
	parents := deque.New[*MerkleNode]()
	for currentNode.left != nil || currentNode.right != nil {
		parents.PushBack(currentNode)
		if currentNode.right != nil {
			currentNode = currentNode.right
		} else {
			currentNode = currentNode.left
		}
	}

	// let's the party started
	// 2 cases - odd and even number of leaves
	isEven := len(mt.leaves)%2 == 0
	parent := parents.PopBack()
	if isEven { // time to promote node
		counter := 0
		for parent.right != nil {
			parent = parents.PopBack()
			counter++
		}
		p := parent
		for i := 0; i < counter; i++ {
			newNode := mt.NewNode(leaf, nil)
			if i == 0 {
				p.right = newNode
				p = p.right
			} else {
				p.left = newNode
				p = p.left
			}
			p.left = leaf
		}
	} else {
		// odd! time to create new node!
		parent.right = leaf
		parent.value = mt.digestNodes(parent.left.value, parent.right.value)
	}

	// re-compute hashes
	for parents.Len() > 0 {
		parent = parents.PopBack()
		if parent.right != nil {
			parent.value = mt.digestNodes(parent.left.value, parent.right.value)
		} else {
			parent.value = parent.left.value
		}
	}

	mt.leaves = append(mt.leaves, leaf)
	mt.rootHash = mt.rootNode.value
	return mt.rootHash, nil
}

func (mt *MerkleTree) Delete(entry *MerkleEntry) ([32]byte, error) {
	// only 1 node
	if len(mt.leaves) == 1 {
		mt.rootNode = nil
		mt.rootHash = [32]byte{}
		mt.depth = 0
		mt.leaves = []*MerkleNode{}
		return mt.rootHash, nil
	}

	// case 2 nodes
	if len(mt.leaves) == 2 {
		if mt.entriesEqual(entry, mt.rootNode.left.leaf) {
			mt.rootNode = mt.rootNode.right
			mt.rootHash = mt.rootNode.value
			mt.depth = 1
			mt.leaves = []*MerkleNode{mt.rootNode}
			return mt.rootHash, nil
		}
		mt.rootNode = mt.rootNode.left
		mt.rootHash = mt.rootNode.value
		mt.depth = 1
		mt.leaves = []*MerkleNode{mt.rootNode}
		return mt.rootHash, nil
	}

	// deleting the only node on right side
	if int(math.Pow(2, float64(mt.depth-1))/2)+1 == len(mt.leaves) &&
		mt.digestEntry(entry) == mt.rootNode.right.value &&
		len(mt.leaves) > 2 {
		mt.rootNode = mt.rootNode.left
		mt.rootHash = mt.rootNode.left.value
		mt.depth = mt.depth - 1
		for i, leaf := range mt.leaves {
			if mt.entriesEqual(leaf.leaf, entry) {
				mt.leaves = append(mt.leaves[:i], mt.leaves[i+1:]...)
				return mt.rootHash, nil
			}
		}
	}

	var leaves []*MerkleNode
	var pendingLeaves []*MerkleNode
	var isOnLeftSide bool
	for i, leaf := range mt.leaves {
		if mt.entriesEqual(leaf.leaf, entry) {
			leaves = mt.leaves[:i]
			pendingLeaves = mt.leaves[i+1:]
			if i < int(math.Pow(2, float64(mt.depth-1))/2) {
				isOnLeftSide = true
			}
			break
		}
	}

	location, found := mt.findLeaf(mt.rootNode, entry)
	if !found {
		return [32]byte{}, errors.New("Something went wrong searching entry")
	}
	target := location[len(location)-1]
	for i := len(location) - 1; i > 0; i-- {
		loc := location[i]
		if loc.left == target {
			loc.left = loc.right
			loc.right = nil
			continue
		}
		loc.right = nil
	}
	if isOnLeftSide {
		mt.rootNode = mt.rootNode.left
		mt.rootHash = mt.rootNode.left.value
		mt.depth = mt.depth - 1
		mt.leaves = leaves

		for _, node := range pendingLeaves {
			mt.Insert(node.leaf)
		}
	}
	return [32]byte{}, nil
}

func (mt *MerkleTree) GenerateMerklePath(entry *MerkleEntry) ([][32]byte, bool) {
	locations, found := mt.findLeaf(mt.rootNode, entry)
	if !found {
		return [][32]byte{}, false
	}

	merklePath := [][32]byte{}
	visitedNode := locations[len(locations)-1].value
	for i := len(locations) - 2; i >= 0; i-- {
		node := locations[i]

		if node.value == visitedNode || node.right == nil {
			visitedNode = node.value
			continue // skip intermediate nodes
		}

		if node.right.value == visitedNode {
			merklePath = append(merklePath, node.left.value)
		} else {
			merklePath = append(merklePath, node.right.value)
		}
		visitedNode = node.value
	}

	return merklePath, true
}

func (mt *MerkleTree) VerifyMerklePath(*MerkleEntry, int, [][32]byte) bool {
	return true
}

func (mt *MerkleTree) findLeaf(node *MerkleNode, entry *MerkleEntry) ([]*MerkleNode, bool) {
	if node == nil {
		return nil, false
	}
	if mt.entriesEqual(node.leaf, entry) {
		return []*MerkleNode{node}, true
	}
	leftPath, found := mt.findLeaf(node.left, entry)
	if found {
		return append([]*MerkleNode{node}, leftPath...), true
	}
	rightPath, found := mt.findLeaf(node.right, entry)
	if found {
		return append([]*MerkleNode{node}, rightPath...), true
	}
	return nil, false
}

func (mt *MerkleTree) entriesEqual(e1, e2 *MerkleEntry) bool {
	if e1 == nil || e2 == nil {
		return false
	}
	if e1.Key != e2.Key {
		return false
	}
	if len(e1.Value) != len(e2.Value) {
		return false
	}
	for i := 0; i < len(e1.Value); i++ {
		if e1.Value[i] != e2.Value[i] {
			return false
		}
	}
	return true
}

func (mt *MerkleTree) digestEntry(entry *MerkleEntry) [32]byte {
	serialized, _ := json.Marshal(entry)
	return sha256.Sum256(serialized)
}

func (mt *MerkleTree) digestNodes(a [32]byte, b [32]byte) [32]byte {
	concatenated := append(a[:], b[:]...)
	return sha256.Sum256(concatenated)
}

func (mt *MerkleTree) Print() {
	fmt.Printf("Merkle Tree (Root hash: %x...):\n", mt.rootHash[:8])
	printNode(mt.rootNode, "", 0, true)
}

func printNode(node *MerkleNode, prefix string, depth int, isLast bool) {
	var marker string
	if isLast {
		marker = strings.Repeat(" ", depth) + "└──"
	} else {
		marker = strings.Repeat(" ", depth) + "├──"
	}
	fmt.Printf("%s%sHash %x...\n", prefix, marker, node.value[:8])
	if node.right == nil && node.left == nil {
		fmt.Printf("%s%s└──Leaf: %s\n", prefix, strings.Repeat(" ", depth+4), node.leaf.Value)
	}
	if !isLast {
		prefix += strings.Repeat(" ", depth) + "│  "
	}
	if node.right != nil {
		printNode(node.right, prefix+strings.Repeat(" ", depth), depth+1, node.left == nil)
	}
	if node.left != nil {
		printNode(node.left, prefix+strings.Repeat(" ", depth), depth+1, true)
	}
}
