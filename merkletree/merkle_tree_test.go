package merkletree

import (
	"crypto/sha256"
	"encoding/json"
	"fmt"
	"strconv"
	"testing"
)

func TestAddOneNode(t *testing.T) {
	merkleTree := NewMerkleTree()
	entry := MerkleEntry{sha256.Sum256([]byte("key1")), []byte("Node1")}
	entryHash, _ := json.Marshal(entry)
	hash := sha256.Sum256(entryHash)
	rootHash, _ := merkleTree.Insert(&entry)

	if rootHash != hash {
		t.Errorf("Should return correct hash when inserting entry to empty merkle tree. Expected hash %x, but got %x", hash, rootHash)
	}
}

func TestAddTwoNodes(t *testing.T) {
	merkleTree := NewMerkleTree()
	entry1 := MerkleEntry{sha256.Sum256([]byte("key1")), []byte("Node1")}
	entry2 := MerkleEntry{sha256.Sum256([]byte("key2")), []byte("Node2")}
	entryHash1, _ := json.Marshal(entry1)
	entryHash2, _ := json.Marshal(entry2)
	h1 := sha256.Sum256(entryHash1)
	h2 := sha256.Sum256(entryHash2)
	expectedHash := merkleTree.digestNodes(h1, h2)
	merkleTree.Insert(&entry1)
	merkleTree.Insert(&entry2)

	fmt.Println()
	fmt.Println("Two Nodes")
	merkleTree.Print()
	fmt.Println()

	if merkleTree.rootHash != expectedHash {
		t.Errorf("Should return correct hash when inserting 2 entries to empty merkle tree. \nExpected hash\n%x,\nbut got \n%x", expectedHash, merkleTree.rootHash)
	}
}

func TestAddThreeNodes(t *testing.T) {
	merkleTree := NewMerkleTree()
	entry1 := MerkleEntry{sha256.Sum256([]byte("key1")), []byte("Node1")}
	entry2 := MerkleEntry{sha256.Sum256([]byte("key2")), []byte("Node2")}
	entry3 := MerkleEntry{sha256.Sum256([]byte("key3")), []byte("Node3")}
	entryHash1, _ := json.Marshal(entry1)
	entryHash2, _ := json.Marshal(entry2)
	entryHash3, _ := json.Marshal(entry3)
	h1 := sha256.Sum256(entryHash1)
	h2 := sha256.Sum256(entryHash2)
	h3 := sha256.Sum256(entryHash3)
	h1h2 := merkleTree.digestNodes(h1, h2)
	expectedHash := merkleTree.digestNodes(h1h2, h3)
	merkleTree.Insert(&entry1)
	merkleTree.Insert(&entry2)
	merkleTree.Insert(&entry3)

	fmt.Println()
	fmt.Println("Three Nodes")
	merkleTree.Print()
	fmt.Println()

	if merkleTree.rootHash != expectedHash {
		t.Errorf("Should return correct hash when inserting 3 entries to empty merkle tree. \nExpected hash\n%x,\nbut got \n%x", expectedHash, merkleTree.rootHash)
	}
}

func TestAddFourNodes(t *testing.T) {
	merkleTree := NewMerkleTree()
	entry1 := MerkleEntry{sha256.Sum256([]byte("key1")), []byte("Node1")}
	entry2 := MerkleEntry{sha256.Sum256([]byte("key2")), []byte("Node2")}
	entry3 := MerkleEntry{sha256.Sum256([]byte("key3")), []byte("Node3")}
	entry4 := MerkleEntry{sha256.Sum256([]byte("key4")), []byte("Node4")}
	entryHash1, _ := json.Marshal(entry1)
	entryHash2, _ := json.Marshal(entry2)
	entryHash3, _ := json.Marshal(entry3)
	entryHash4, _ := json.Marshal(entry4)
	h1 := sha256.Sum256(entryHash1)
	h2 := sha256.Sum256(entryHash2)
	h3 := sha256.Sum256(entryHash3)
	h4 := sha256.Sum256(entryHash4)
	h1h2 := merkleTree.digestNodes(h1, h2)
	h3h4 := merkleTree.digestNodes(h3, h4)
	fmt.Printf("h1h2 %x...\n", h1h2[:8])
	fmt.Printf("h3h4 %x...\n", h3h4[:8])
	expectedHash := merkleTree.digestNodes(h1h2, h3h4)
	merkleTree.Insert(&entry1)
	merkleTree.Insert(&entry2)
	merkleTree.Insert(&entry3)
	merkleTree.Insert(&entry4)

	fmt.Println()
	fmt.Println("Four Nodes")
	merkleTree.Print()
	fmt.Println()

	if merkleTree.rootHash != expectedHash {
		t.Errorf("Should return correct hash when inserting 4 entries to empty merkle tree. \nExpected hash\n%x,\nbut got \n%x", expectedHash, merkleTree.rootHash)
	}
}

func TestAddFiveNodes(t *testing.T) {
	merkleTree := NewMerkleTree()
	entry1 := MerkleEntry{sha256.Sum256([]byte("key1")), []byte("Node1")}
	entry2 := MerkleEntry{sha256.Sum256([]byte("key2")), []byte("Node2")}
	entry3 := MerkleEntry{sha256.Sum256([]byte("key3")), []byte("Node3")}
	entry4 := MerkleEntry{sha256.Sum256([]byte("key4")), []byte("Node4")}
	entry5 := MerkleEntry{sha256.Sum256([]byte("key5")), []byte("Node5")}
	entryHash1, _ := json.Marshal(entry1)
	entryHash2, _ := json.Marshal(entry2)
	entryHash3, _ := json.Marshal(entry3)
	entryHash4, _ := json.Marshal(entry4)
	entryHash5, _ := json.Marshal(entry5)
	h1 := sha256.Sum256(entryHash1)
	h2 := sha256.Sum256(entryHash2)
	h3 := sha256.Sum256(entryHash3)
	h4 := sha256.Sum256(entryHash4)
	h5 := sha256.Sum256(entryHash5)
	h1h2 := merkleTree.digestNodes(h1, h2)
	h3h4 := merkleTree.digestNodes(h3, h4)
	h1h2h3h4 := merkleTree.digestNodes(h1h2, h3h4)
	expectedHash := merkleTree.digestNodes(h1h2h3h4, h5)
	merkleTree.Insert(&entry1)
	merkleTree.Insert(&entry2)
	merkleTree.Insert(&entry3)
	merkleTree.Insert(&entry4)
	merkleTree.Insert(&entry5)

	fmt.Println()
	fmt.Println("Five Nodes")
	merkleTree.Print()
	fmt.Println()

	if merkleTree.rootHash != expectedHash {
		t.Errorf("Should return correct hash when inserting 4 entries to empty merkle tree. \nExpected hash\n%x,\nbut got \n%x", expectedHash, merkleTree.rootHash)
	}
}

func nLeafTree(n int) *MerkleTree {
	merkleTree := NewMerkleTree()
	for i := 0; i < n; i++ {
		entry := MerkleEntry{sha256.Sum256([]byte("key" + strconv.Itoa(i+1))), []byte("Node" + strconv.Itoa(i+1))}
		merkleTree.Insert(&entry)
	}
	return merkleTree
}

func TestTenNodes(t *testing.T) {
	merkleTree := nLeafTree(10)

	fmt.Println()
	fmt.Println("Ten Nodes")
	merkleTree.Print()
	fmt.Println()
}

func TestDeleteHangingRight(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 3 on 3 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(3)
	entry := merkleTree.rootNode.right.left
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println("Node 3 deleted")
	merkleTree.Print()
}

func TestDeleteHangingRight2(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 5 on 5 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(5)
	entry := merkleTree.rootNode.right.left.left
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println()
	fmt.Println("Node 5 deleted")
	merkleTree.Print()
}

func TestDeleteLeafLeftSide(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 3 on 5 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(5)
	entry := merkleTree.rootNode.left.right.left
	fmt.Printf("entry %s\n\n", string(entry.leaf.Value))
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	// fmt.Println()
	// fmt.Println("Node 5 deleted")
	// merkleTree.Print()
}
