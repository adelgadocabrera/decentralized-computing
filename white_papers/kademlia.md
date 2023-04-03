# Kademlia: A Peer-to-peer Information System Based on the XOR Metric

## Overview

Kademlia is a distributed hash table (DHT) protocol that is designed to enable efficient decentralized data storage and retrieval in peer-to-peer (P2P) networks. The protocol was proposed by Petar Maymounkov and David Mazières in 2002. 

The key idea behind Kademlia is to use a distributed routing table to efficiently locate data items in the network. Each node in the network maintains a routing table that contains information about other nodes in the network. The routing table is organized into a binary tree structure, with each level of the tree corresponding to a different prefix length of the node IDs. The routing table contains information about the nodes that are closest to the local node, based on the XOR distance metric.

When a node wants to store a data item in the network, it first computes the hash of the item to obtain its ID. It then uses the ID to locate the set of nodes in the network that are closest to the item. The node then contacts these nodes and stores the item with one or more of them. When a node wants to retrieve a data item from the network, it performs a similar search to locate the nodes that are closest to the item. It then contacts these nodes and retrieves the item from one or more of them.

Kademlia provides several important features that make it suitable for large-scale P2P networks. 
- First, it has a low communication overhead, as each node only needs to maintain information about a small number of other nodes in the network. 
- Second, it is highly fault-tolerant, as the network can continue to function even if a large number of nodes fail. 
- Third, it provides good load balancing, as the distribution of data items in the network is determined by the IDs of the items themselves.

The Kademlia protocol has been used in several popular P2P networks, including BitTorrent, eDonkey, and Gnutella2. It has also been implemented in several open-source software projects, such as the Kad network in the eMule file-sharing client.

## Routing Table

A node's routing table consists of multiple $k$-buckets, where each $k$-bucket contains information about other nodes in the network that share a common prefix with the node's own ID.

- The number of bits that are shared in common between the node's ID and another node's ID is called the "prefix length". Each $k$-bucket is responsible for storing information about nodes with a particular prefix length. For example, the $k$-bucket with prefix length $i$ will store information about nodes whose IDs share the first $i$ bits with the node's own ID.

- Each $k$-bucket has a maximum size, which is denoted by $k$. When a node wants to add a new node to its routing table
	1. First it determines the prefix length of the new node's ID.
	2. Then looks for the $k$-bucket that corresponds to that prefix length. 
		- If the k-bucket is not full, the node is added to the $k$-bucket. 
		- If the k-bucket is full, the node pings the last node in the $k$-bucket to see if it is still reachable. 
			- If it is not reachable, the new node replaces it in the k-bucket.
			- If it is reachable, the new node is discarded.

- The $k$-buckets in a node's routing table are ordered based on the last time that a node in the k-bucket was contacted. 
	- The most recently contacted nodes are at the beginning of the k-bucket, while the least recently contacted nodes are at the end. 
 
- When a node wants to find a node in the network, it first looks in the $k$-bucket corresponding to the prefix length of the target node's ID. 
	- If the $k$-bucket contains nodes, the node contacts the $k$ closest nodes in the $k$-bucket. 
	- If the $k$-bucket does not contain $k$ nodes, the node contacts nodes from other $k$-buckets in the routing table, starting with the $k$-buckets closest to the target node's prefix length.

## k-buckets Advantages

$k$-buckets have several advantages:
1. First, they provide a compact and efficient data structure for organizing and maintaining a node's routing table. 
2. Second, they enable efficient lookups by allowing a node to quickly locate the closest nodes to a target ID. 
3. Third, they provide a mechanism for handling churn in the network, as nodes that are no longer reachable can be replaced with new nodes. 

