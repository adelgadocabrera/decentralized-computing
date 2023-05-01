# Distributed Systems Essay

Why is it hard to build distributed systems? 
_Source Designing Data Intensive Applications by Martin Kleppmann._

- Unreliable networks: networks can experience latency, drop connections, packet loss, causing communication problems between nodes 
- Message delays: messages may be delayed or arrive out of order causing coordination problems between nodes
- Unreliable (real) clocks: clock synchronization is difficult, may cause skewed time and make it unreliable to determine causality between events
- Node failures: software and/or hardware issues can cause node failures which affects the availability (see below) and fault tolerance. Nodes are also exposed to malicious attacks
- Inconsistent data replication: inconsistencies may arise specially when replicating asynchronously 
- Concurrency issues: conflicts may occur during concurrent reads and writes leading to dirty reads, dirty writes and read skew
- Network partitions: communication between nodes may be disrupted or compromised, causing partitions that affect availability and consistency. 
- Cascading failures: failures on one part of the system can cause failures in another, potentially leading to system outages 

> _A distributed system is one in which the failure of a computer you didn't even know existed can render your own computer unusable - Leslie Lamport_

// Trying to go from more broad and abstract concepts to more specific 

Down below, we will begin by addressing the broader aspects and overview of distributed systems, including the trade-offs and challenges involved. Then, we will delve into more specific cases and techniques utilized for resolving problems in distributed systems.

The CAP theorem is a foundation for understanding the trade-offs in distributed systems. It highlights that only two of the three properties – Consistency, Availability, and Partition tolerance – can be achieved simultaneously. During the semester, various white papers have been reviewed, which often emphasize finding a balance between consistency and availability. Many of these papers do not focus on partition tolerance, opting instead to prioritize the other two aspects of the CAP theorem. 
// TO DO: I have to find the video where they propose that you cannot give up the P in CAP theorem, as it would bring the system useless which is a very compelling idea.

High availability and high consistency are two important concepts in distributed systems. High availability refers to the design of systems that minimize downtime and ensure that services remain accessible. High consistency, on the other hand, guarantees data accuracy/integrity across all nodes. Consistency, from a user perspective, means that data appears consistent, even if it is not actually consistent across all machines at a given time. 

Causality in distributed systems is a critical, as it helps track events accurately. Logical clocks are used to represent the order of events, while real clocks aim to provide real time, which turns out not to be taken for granted as it may be harder to calculate than it initially seemed, clock skews, network delays with NTP etc. These clocks allow for proper coordination of actions/events within the system, which is particularly important when considering the consistency of data across multiple nodes.

Synchronous and asynchronous replication. Synchronous communication prioritizes data consistency at the expense of latency, while asynchronous replication prioritizes speed over consistency. Both methods have their own advantages and disadvantages, making the choice dependent on the specific requirements of a system. 
// insert diagram showing how sync communication can stall the entire system
// insert diagram to show how async communication can cause dirt reads

Dirty reads and dirty writes are phenomena that can lead to potential inconsistencies within a distributed system. These occur when transactions access uncommitted or outdated data, resulting in read skew. Databases use various approaches to manage concurrent reads and writes, such as locking and versioning techniques (Serializable Snapshot Isolation, 2-Phase Locking, etc). These strategies ensure that transactions are properly coordinated and that data remains consistent.

Two-phase locking and two-phase commit are strategies employed in distributed systems to maintain consistency and coordinate transactions. 

Consensus algorithms, such as Paxos, aim to achieve agreement among distributed nodes despite potential failures or communication issues. These algorithms are pivotal for maintaining the reliability and accuracy of distributed systems.

Distributed hash tables like Chord offer efficient and scalable key-value storage solutions. 

