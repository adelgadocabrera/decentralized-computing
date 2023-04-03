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

func TestDeleteSmallTree2(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 1 on 2 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(2)
	entry := merkleTree.rootNode.left
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println()
	fmt.Println("Node 1 deleted")
	merkleTree.Print()
}

func TestDeleteSmallTree(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 2 on 2 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(2)
	entry := merkleTree.rootNode.right
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println()
	fmt.Println("Node 2 deleted")
	merkleTree.Print()
}

func TestDeleteHangingRight(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 3 on 3 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(3)
	entry := merkleTree.rootNode.right.left
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println()
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
	fmt.Println()
	fmt.Println("Node 3 deleted")
	merkleTree.Print()
}

func TestDeleteLeafRightSide(t *testing.T) {
	fmt.Println()
	fmt.Println("Delete leaf 6 on 6 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(6)
	entry := merkleTree.rootNode.right.left.right
	fmt.Printf("entry %s\n\n", string(entry.leaf.Value))
	merkleTree.Print()
	merkleTree.Delete(entry.leaf)
	fmt.Println()
	fmt.Println("Node 6 deleted")
	merkleTree.Print()
}

func TestMerklePath(t *testing.T) {
	fmt.Println()
	fmt.Println("Merkle Path on 6 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(6)
	entry := merkleTree.rootNode.right.left.right
	fmt.Printf("Entry %s\n\n", string(entry.leaf.Value))
	merkleTree.Print()
	merklePath, ok := merkleTree.GenerateMerklePath(entry.leaf)
	if !ok {
		fmt.Println()
		fmt.Println("Something went wrong calculating merkle path")
		fmt.Println()
	}
	fmt.Println()
	fmt.Println("Merkle Path:")
	for i := 0; i < len(merklePath); i++ {
		fmt.Printf("Node %d: %x...\n", i, merklePath[i][:8])
	}
	fmt.Println()
	merkleTree.Print()
}

func TestMerklePathBig(t *testing.T) {
	fmt.Println()
	fmt.Println("Merkle Path on 6 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(15)
	entry := merkleTree.rootNode.left.right.right.left
	fmt.Printf("Entry %s\n\n", string(entry.leaf.Value))
	merklePath, ok := merkleTree.GenerateMerklePath(entry.leaf)
	if !ok {
		fmt.Println()
		fmt.Println("Something went wrong calculating merkle path")
		fmt.Println()
	}

	first := merkleTree.rootNode.left.right.right.right.value
	second := merkleTree.rootNode.left.right.left.value
	third := merkleTree.rootNode.left.left.value
	forth := merkleTree.rootNode.right.value

	merkleTree.Print()

	if first != merklePath[0] {
		t.Errorf("\nFirst node in merkle path incorrect, \n > expected %x... \n > got %x\n", first[:8], merklePath[0][:8])
	}
	if second != merklePath[1] {
		t.Errorf("\nSecond node in merkle path incorrect, \n > expected %x... \n > got %x\n", second[:8], merklePath[1][:8])
	}
	if third != merklePath[2] {
		t.Errorf("\nThird node in merkle path incorrect, \n > expected %x... \n > got %x\n", third[:8], merklePath[2][:8])
	}
	if forth != merklePath[3] {
		t.Errorf("\nForth node in merkle path incorrect, \n > expected %x... \n > got %x\n", forth[:8], merklePath[3][:8])
	}
}

func TestMerklePathBig2(t *testing.T) {
	fmt.Println()
	fmt.Println("Merkle Path on 6 node merkle tree")
	fmt.Println()
	merkleTree := nLeafTree(12)
	entry := merkleTree.rootNode.right.left.right.right
	fmt.Printf("Entry %s\n\n", string(entry.leaf.Value))
	merklePath, ok := merkleTree.GenerateMerklePath(entry.leaf)
	if !ok {
		fmt.Println()
		fmt.Println("Something went wrong calculating merkle path")
		fmt.Println()
	}

	first := merkleTree.rootNode.right.left.right.left.value
	second := merkleTree.rootNode.right.left.left.value
	third := merkleTree.rootNode.left.value

	merkleTree.Print()

	if first != merklePath[0] {
		t.Errorf("\nFirst node in merkle path incorrect, \n > expected %x... \n > got %x\n", first[:8], merklePath[0][:8])
	}
	if second != merklePath[1] {
		t.Errorf("\nSecond node in merkle path incorrect, \n > expected %x... \n > got %x\n", second[:8], merklePath[1][:8])
	}
	if third != merklePath[2] {
		t.Errorf("\nThird node in merkle path incorrect, \n > expected %x... \n > got %x\n", third[:8], merklePath[2][:8])
	}
}
