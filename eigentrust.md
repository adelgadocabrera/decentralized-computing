# EigenTrust White Paper Summary

The EigenTrust paper presents a novel algorithm for reputation management in Peer-to-Peer (P2P) networks, aiming to address the issue of trust and malicious behavior in such systems. The authors propose the EigenTrust algorithm, which utilizes a decentralized approach to compute a global trust value for each peer in the network, based on the local trust values assigned by its neighbors. The algorithm is designed to be robust, scalable, and resilient against malicious peers attempting to subvert the system.

## Introduction

The authors discuss the importance of trust in P2P networks, as these systems often lack centralized authority and control, making them vulnerable to malicious behavior such as distributing inauthentic files or launching attacks. They argue that a robust and scalable reputation management system can help mitigate these issues by assigning trust values to peers based on their past behavior.

## EigenTrust Algorithm

The EigenTrust algorithm consists of two main components:

### a. Local Trust Values
Each peer i maintains a local trust value, c(i, j), for each of its neighbors j. This value is a measure of peer i's direct experience with j and is computed based on the number of satisfactory transactions i has had with j, normalized by the total number of transactions between them.

### b. Global Trust Values
To compute the global trust value for each peer, the EigenTrust algorithm aggregates local trust values from all peers in the network. The global trust value T(j) for peer j is calculated as the sum of the products of local trust values and the corresponding global trust values of the neighbors:

<center>T(j) = ∑ c(i, j) * T(i)</center>

The authors show that calculating global trust values in this manner converges to the principal eigenvector of the matrix of local trust values, hence the name "**EigenTrust**".

## Aggregation of Trust Values

The paper presents two methods for aggregating trust values: a centralized approach and a distributed approach.

### a. Centralized Approach
A single trusted authority collects all local trust values, computes the global trust values, and distributes them to the peers. While simple and efficient, this method introduces a single point of failure and contradicts the decentralized nature of P2P networks.

### b. Distributed Approach
The authors propose a distributed algorithm for calculating global trust values based on a random walk with restarts. Peers iteratively query their neighbors for trust values, gradually updating their estimates until convergence is reached. This approach is more resilient to attacks and aligns better with the decentralized nature of P2P networks.

#### Score Managers
Score managers are an essential component of the distributed approach to calculating global trust values in the EigenTrust algorithm. They help prevent peers from manipulating their own trust scores by assigning the responsibility of trust calculation and storage to other peers.

Each peer i in the network is assigned a score manager, which is another peer in the network, responsible for maintaining and updating the global trust value T(i) for peer i. The score manager is selected based on a deterministic function, such as a hash function applied to the peer's identifier. This ensures that the assignment of score managers is consistent across the network and difficult for malicious peers to control.

The distributed trust calculation process involves the following steps:

1. Peers send their local trust values to their corresponding score managers.
2. Score managers aggregate the local trust values received from all peers and use them to compute and store the global trust values for the peers they manage.
3. When a peer i needs the global trust value of another peer j, it requests the value from the score manager of peer j.
4. The score manager of peer j responds with the requested global trust value.

By employing score managers, the EigenTrust algorithm effectively prevents peers from calculating and storing their own global trust values, making it more challenging for malicious peers to manipulate the system. The score manager concept ensures that trust values are calculated and managed by other peers in the network, thus promoting honesty and reducing the risk of collusion.

## Security Analysis and Experimental Results

The paper analyzes the EigenTrust algorithm's resilience against malicious peers who try to subvert the system by providing false trust values or colluding with other malicious peers. The authors show that the algorithm effectively isolates such peers and prevents them from having a significant impact on global trust values.

Experimental results with a simulated P2P network demonstrate that the EigenTrust algorithm effectively reduces the number of downloads of inauthentic files, scales well with the size of the network, and converges quickly.


## Conclusion

The EigenTrust paper presents a novel reputation management algorithm for P2P networks that provides a robust and scalable solution for assessing trust in decentralized systems. The EigenTrust algorithm effectively mitigates the impact of malicious behavior in P2P networks, making it a valuable contribution to the field of trust and reputation management.

