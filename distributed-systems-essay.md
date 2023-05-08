# Distributed Systems Essay

Table of contents:
1. Introduction
2. Single machine 
3. Introducing multiple machines (beginnings of a distributed system)
4. Motivation for distributed fault-tolerant systems
5. Interconnection between machines
6. Communication methods in distributed systems
7. Problems arising from relying on the internet
8. Distributed systems concepts and challenges
9. Dealing with failures in distributed systems

## 1. Introduction 

The internet has been around for quite sometime now, and many people have forgotten it wasn't once as reliable as it is now nor are the machines that participate in it. The proposal for this essay is not to cover all the topics related to distributed systems but to shed some light over the complexities that arise in distributed systems, learn why we build such complicated systems, and what problems arise in the process. 

As mentioned, the process is going to follow a bottom-up approach, starting from the simplest form which is running a single machine and evolving it into a complex system. 

## 2. Single machine 
In contrast to what many people think belief nowadays, in the world of microservices and service-oriented architecture a system composed by only one machine offers many benefits, as well as limitations. 

### Advantages of running a single machine
a. Simplicity. Makes up for an easier setup and management, reducing the effort for configuring, maintaing and troubleshooting a single machine. 

b. Cost effectiveness. Generally speaking it is indeed cheaper to buy the components of a single machine. Although in the other hand, consumer electronics have become very affordable and have a decent amount of power. This made distributed computing more affordable.

c. Data consistency. Data has become the most pivotal piece, everything evolves around the data. Handling data in a single machine is is far more simple than having it distributed in different machines. If we were to simplify this even further and assume there was only one thread, data would always remain consistent and events will always be causal. On the other hand it is easy to foresee this piece to become a huge bottleneck.   

d. Performance. There is nothing as performant as running everything locally, data access and information sharing between processes are shared locally, avoiding all possible network delays and latencies that exist in a distributed storage system.  

e. Security. A single machine will be less exposed to attacks and the lack of information sharing with other machines makes it impossible to intercept any messages.  

### Limitations and physical problems a single machine may suffer from
a. Limited resources (CPU, memory, storage). The main problem is all the resources are shared among all running applications. Therefore a high workload on one app may affect the overall performance, slow response times and lead even to crashes.

b. Scalability issues (inability to handle increased workload). This is what happens when the resources of the machine are maxed out. There is no way of scaling, besides vertically. Although consumer electronics have become more affordable and even high end components are not as expensive, there is a limit on how many cores a CPU may have or how much RAM or disk space a machine may have. This can become very expensive and we are not even accounting for downtime for upgrading. 

c. Fault tolerance concerns (single point of failure). By far the most important concern when running a single application. Single point of failure. It can happen due to many reasons, including hardware failures, software failures and environmental issues.

- Hardware failures (disk, CPU, memory, power supply). It is very common for hard drives to fail, RAM sticks to become faulty, or any other component may fail. It is common to configure the disks in RAID for redundancy, have more than one CPU or even to have more than one power supply too. Nonetheless this won't completely guarantee complete prevention from hardware failures. 

- Software failures (bugs, crashes, memory leaks). There are a myriad of things that can go wrong within a single machine and they are not always intuitive. There could be a systematic error withing the OS, a software bug that causes an application to crash given a certain input, cascading failures where a small fault my trigger a series of consecutive failures. 

- Environmental issues (power outages, temperature, humidity). Despite hardware redundancy, a machine won't be able to survive a power outage, or even worse, earthquakes or any other unpredictable natural disasters. Temperature and humidity will also greatly affect components lifespan.

## 3. Introducing multiple machines (beginnings of a distributed system)


    a. Increased resources and capacity
    b. Improved fault tolerance (redundancy)
    c. Initial challenges in coordinating and managing resources

## 4. Motivation for distributed fault-tolerant systems
    a. Addressing single point of failure
    b. Providing redundancy and failover capabilities
    c. Ensuring data durability and availability

## 5. Interconnection between machines
    a. Networking basics (LAN, WAN, Internet)
    b. Network latency and bandwidth limitations
    c. Data consistency and synchronization challenges

## 6. Communication methods in distributed systems
    a. Synchronous communication (blocking, immediate response)
    b. Asynchronous communication (non-blocking, delayed response)
    c. Message passing and remote procedure calls (RPC)

## 7. Problems arising from relying on the internet
    a. Network partitions and failures
    b. Security concerns (authentication, authorization, encryption)
    c. Performance variability due to distance and network congestion

## 8. Distributed systems concepts and challenges
    a. Consistency models (eventual consistency, strong consistency)
    b. Replication strategies (active/passive, active/active)
    c. Load balancing and resource allocation

## 9. Dealing with failures in distributed systems
    a. Fault detection and monitoring
    b. Fault recovery and failover mechanisms
    c. Data backups and disaster recovery planning

--- 

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

