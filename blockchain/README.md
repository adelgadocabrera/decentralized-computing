# Blockchain 

Blockchain is a distributed data structure with special properties, such as tamper-resistance. Most recently it gained interests worldwide as a backbone of cryptocurrency. This project covers public-key cryptography, one-way hash functions, Merkle tree, proof-of-work, proof-of-stake, Byzantine-fault tolerance and consensus algorithms.

## Techniques used 

- Cryptographic hash functions 
- Merkle Tree (for data integrity and membership test) 
- Merkle Patricia Trie 
- Consensus
- Crash tolerant consensus 
- Public-key cryptography 
- Digital signature for authentication 

## Project Components  
The main components of the blockchain implementation include:

- **Block**: A block contains a set of transactions, a timestamp, and a hash of the previous block.

- **Blockchain**: The blockchain consists of a chain of blocks and uses proof-of-work as its consensus mechanism. Nodes compete to solve a computational puzzle, and the winner adds a new block to the chain.

- **Transaction**: A transaction contains information about the sender, recipient, and amount transferred.

- **Wallet**: A wallet contains a public key and a private key. Public keys are used to receive funds, while private keys are used to sign transactions and prove ownership of funds.

- **Mining**: To add new blocks to the blockchain, nodes perform a computational task called mining. Mining requires nodes to find a hash that meets a certain criteria, which becomes more difficult as more nodes join the network.

- **Networking**: Nodes communicate with each other over a peer-to-peer (P2P) network. When a node adds a new block to the chain, it broadcasts the new block to all other nodes in the network.

- **Consensus**: To ensure that all nodes agree on the state of the blockchain, a consensus mechanism is used. In this implementation, we use proof-of-work as the consensus mechanism.

- **Smart contracts (optional)**: support for smart contracts. Programming language and virtual machine that allows users to write and execute code on the blockchain.
