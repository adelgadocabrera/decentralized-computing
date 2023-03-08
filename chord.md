# Chord: A Scalable Peer-to-peer Lookup Service for Internet Applications

## Summary 

### Intro

Chord DHT (Distributed Hash Table) is a distributed lookup protocol for storing and retrieving key-value pairs in a peer-to-peer network. It is a ring-based overlay network that partitions data across multiple nodes designed to ensure efficient lookups and retrieval of data even in the presence of of node failures or network partitions. 

It is a popular DHT implementation and fairly used in various applications such as p2p file sharing or distributed databases.

### The 5 pillars of Chord:
- **Load balance** - Chord relies on consistent hashing algorithm to assign keys to nodes in the network. Acts as a natural load balancer. To ensure all peers actually participate in the storage of keys an implementation in conjunction with virtual nodes can take place.
- **Decentralization** - no node/peer is more important than any other. *Same software running in all peers*.
- **Scalability** - Chord makes use of *finger tables* for efficient lookups. *Finger tables* are data structures to keep track of other nodes in the network. The i<sub>th</sub> entry in a node's finger table contains the IP address and identifier of the node that succeeds the node by 2<sup>(i-1)</sup> on the identifier circle. Usage of *finger tables* results in $O(log N)$ lookups.
- **Availability** - peers can join and leave (and crash/fail) the network. Chord leverages a set of *finger tables* (see **Scalability**) on the event of changes while maintaining an operational system (lookups should still work while peers join/leave). The automatic adjustment although not immediate is fast and always converges to a estable state.
- **Flexible naming** - Chord protocol does not impose any restrictions on the format/structure of the keys used for lookups. As a result, applications that use Chord can use any naming convention without being constraint by Chord. 

### Finger tables and lookups 

A peer maintains about a fixed number of nodes - *finger table*. The general equation for calculating the i<sub>th</sub> entry in a node's *finger table* is: 

$succ[i] = (n + 2^{(i-1)}) \times mod\,\,2^m$

Where:
- $n$ is the identifier of the current node 
- $m$ is the number of bits in the identifier space 
- $i$ is the index of the entry in the *finger table* $(1 <= i <= m)$ 
- and $mod$ is the modulo operator

Each node maintains entries for its own identifier and the identifiers of its successors in the *finger table*

### Calculating the finger table

A peer calculates the *finger table* in $O(log N)$ time complexity, where $N$ is the total number of nodes in the system. Here is how it works for the case of a peer joining the network:

1. Peer joins network and creates *finger table* with its own successor node. 

2. Peer starts *finger table* calculation with information about the other nodes in the system. Peer iteratively queries other nodes in the system to find its successors for each entry in the *finger table*: 
	- Query successor - peer first queries own successor for i<sub>th</sub> entry in the *finger table*. 
		- If successor's identifier is greater than or equal to the *finger table* entry, the entry is updates to contain the successor's identifier. 
		- If successor's identifier is less than the *finger table* entry, the node forwards the query to the node in the system whose identifier is closest to the target identifier of the *finger table* entry.
	- Repeat the process until it has updated all entries in the *finger table*.

Number of entries in *finger table* is $O(logN)$ and each query in step 2 takes $O(logN)$ time complexity. Overall time complexity for updating *finger table* results in $O(log^2N)$. There are techniques to reduce it to $O(logN)$. 

## Questions

**What are the potential gains and losses of transitioning from a single trusted server with complete knowledge to a decentralized network of peer-to-peer nodes?**

Here are some examples of gains and losses.

Gains:
1. Decentralization - in the case from a single source of truth server, it can be a huge bottleneck and additionally a single point failure. There is also the benefit of transparency - every node should see the same data
2. Availability - following on the previous point, not only there is no single point of failure but data is distributed among many nodes which could mean being able to handle more requests
4. Scalable - in a Chord-like system where lookups are efficient it allows for great scalability without the system being overwhelmed and performance loss
5. Resilience - even if a few nodes fail the overall system should stay operational

Losses:
1. Implementing a decentralized system is way more complicated than a centralized controlled server 
2. Additionally, there is no central authority therefore security and integrity of data is a great concern
3. Single source of truth can handle consistency better (only one copy of the data) whereas in a decentralized system consensus algorithms should take place to ensure consistency

**Can Chord be combined with OCCULT?** 

The main problem I see is, if the distributed system grows very large, which Chord is designed to, keeping track of the causal timestamps per shard is very space costly. Which in OCCULT they solve by using compression. 

But besides that it seems like it is feasible. The great thing about OCCULT was that clients have to see causal reads/writes from their end, therefore ring topology like Chord could actually work if combined with using replicas and partitioning properly. 

The result could be something similar to Dynamo.

**How to prevent malicious behavior when implementing Chord?**

1. Authentication - nodes have to use cryptographic keys to prevent unauthorized access ()
2. Reputation system (similar to Bitswap in IPFS?) - have a reputation score among peers. Be brutal with peers with low score, kick them out! 
3. Proper load balancing - ensure workload is "evenly" distributed among peers. Try to avoid DoS attacks
4. Redundancy. Help the overall performance of the system by adding replicas
