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

## Block

### Header
- A block's header contains some information about the miner, the block, and the current state of the network. Every block header contains the following fields:

- Parent block's hash: It is the Keccak 256-bit hash of its parent block's header. It is used to chain this block to the previous block making the blockchain more secure.

- Uncle block's hash: It is the Keccak 256-bit hash of the list of uncle block headers present in the block's body.

- Beneficiary address: It is the 160-bit account address of the miner of this block to which all the mining fees from this block will be transferred.

- State root: It is the Keccak 256-bit hash of the root node of the state trie after all the transactions are executed and finalizations are applied.

- Transaction root: It is the Keccak 256-bit hash of the root node of the Merkle root trie populated by all the transactions present in that block's body.

- Receipt root: The Keccak 256-bit hash of the root of the Merkel root trie that is populated by the receipts of all the transactions in that block.

- Logs bloom: A Bloom filter consisting of indexable log entries from the receipt of each transaction from the list of transactions present in the body of the block.

- Block number: The length of the blockchain or the number of blocks in the blockchain. A positive whole number equals the number of ancestor blocks of this block where the genesis block is block 0.

- Gas limit: The current upper limit on the gas used in the block.

- Gas used: Total gas used in transactions in this block.

- Difficulty: It is the difficulty of the network to mine this block. Miners' have to mine a block whose header's hash is less than the network's difficulty at that time. Miners do this by changing the nonce value until they find a value with which the hash of the block is less than the network's difficulty.

- Mix hash: A unique identifier for the block. When combined with the nonce proves that a sufficient amount of work has been done by the miner (proof of work).

- Timestamp: The time in Unix format when the block was mined.

- Base fee per gas: The minimum fee required per gas for the transaction to be included in the block.

- Extra data: A byte array containing data relevant to this block. This must be less than 32 bytes.

### Body
The body of an Ethereum block contains the list of uncle block headers and a list of transactions (both are described below).

- Uncle block headers: This is a list of uncle block headers (same format as a block header described above).

- Transactions: This is a list of actual Ethereum transactions stored in the block.


