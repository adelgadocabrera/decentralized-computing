# IPFS - Content Addressed, Versioned, P2P File System (Draft 3)

## What is IPFS 

In contrast with other white papers, IPFS does not try to reinvent the wheel, but instead uses already existing distributed systems techniques and data structures and combines them to create a peer-to-peer (globally) distributed file system. IPFS aims to provide high-throughput with no single point of failure.

IPFS can be summarized with this 15 word definition directly extracted from the white paper: "(..) IPFS could be seen as a single BitTorrent swarm, exchanging objects within one Git repository". Additionally, IPFS also uses Distributed Hash Tables and Self-certified Filesystems (SFS).

## IPFS Design

IPFS presents a new way of storing large amounts of data, and even an alternative to web hosting. It all comes down to the 7 pillars: 
- **Identities**
	- (Kademlia inspired)
	- Handles node identity 
	- Every node has an ID
	- Node ID can be re-used from previous sessions or generated on new session (see **Exchange** to learn about the trade-offs)
	- On peer-to-peer connection public keys are exchanged for authenticity 
- **Network**
	- Manages the underlying connections to other peers. What does this mean?
	- IPFS is responsible of ensuring a reliable connection in any transport protocol 
	- Protocols and addresses are stored as ` multiaddr ` 
		- Example ` /ip4/10.20.30.40/sctp/1234/ `
- **Routing**  
	- Handles peer discovery and objects (data) offered by peers
	- IPFS defaults this task to a Distributed Hash Table actually an interface of the different methods for peer and data discovery are exposed. Therefore the routing system could be swapped for a different one
- **Exchange**
	- Manages the exchange of blocks (data) between nodes. 
	- Problem: How do you incentivize it? Remember BitTorrent had precisely this problem, as leechers would try to download as much data while sharing the least amount (see **BitSwap** below)
	- Whereas BitTorrent clients would download chunks of data belonging to a particular torrent, IPFS only understands blocks 
	- Any client can provide or request any block
	- Nodes have a _want_list_ and a _have_list_ of blocks
	- **BitSwap** - protocol created to generate credit and incentivize clients to share blocks.
		- The main idea is "lenient to debts between nodes that have previously exchanged lots of data successfully, and merciless to unknown, untrusted nodes"
		- Nodes keep a ledger of sent/received bytes for every connection
		- Two nodes upon connection compare this ledger 
			- If there is a match they seem to trust each other (specially if sent/received bytes). Start block sharing
			- If mismatch - either client can decide to close connection as they can come across as potential malicious agent
- **Objects - Merkle DAG**
	- Objects are connected in a directed acyclic graph (DAG) where links between objects are cryptographic hashes of the objects it contains (for more information, search Merkle DAG)
	- Provides
		- Content addressing which is a crucial part of IPFS 
		- Cannot tamper content as it is cryptographically verified
		- Cannot duplicate data, as if a client intends to upload already existing data the hash will be the same
	- IPFS provides a very basic data structure (basically it has _data_ and _children_ properties as you would find in any tree data structure) for its objects, abstract and flexible enough so that clients can create their own data structures - it is also easy to traverse
- **Files**
	- Git inspired file system with versioned content:
		- _block_ - block of data 
		- _list_ - collection of blocks or other lists
		- _tree_ - collection of blocks, lists or other trees
		- _commit_ - version snapshot 
	- Optimizations for lookup performance include **tree caching** and **flattened trees**
- **IPNS - Naming and Mutable State**
	- Data in IPFS is _content-addressed_ and _immutable_ - OBJECTS ARE PERMANENT
	- Once you upload a piece of data you cannot edit it - if you host a website, in order to re-upload it with new changes you'll get a completely new CID (Content ID) in return
	- Two methods for accessing content
		- **Interplanetary Name System IPNS** allows content-addressable links to be accessed by a mutable domain (its essentially pointers - IPNS names - to pointers - IPFS CIDs)
		- **Mutable File System (MFS)**, a file system built on top of IPFS that allows to create, remove, move and edit MFS files 

