# Distributed Systems Essay
> _A distributed system is one in which the failure of a computer you didn't even know existed can render your own computer unusable - Leslie Lamport_

Table of contents:
1. Introduction
2. Single machine architecture
3. Motivation for distributed fault-tolerant systems
4. Introducing multiple machines 
5. Interconnection between machines
6. Problems arising from relying on the internet
7. Communication methods in distributed systems
8. Distributed systems concepts and challenges
9. Transactions 
10. Dealing with failures in distributed systems

## 1. Introduction 

The internet has been around for quite sometime now, and many people have forgotten it wasn't once as reliable as it is now nor are the machines that participate in it. The proposal for this essay is not to cover all the topics related to distributed systems but to shed some light over the complexities that arise in distributed systems, learn why we build such complicated systems, and what problems arise in the process. 

As mentioned, the process is going to follow a bottom-up approach, starting from the simplest form which is running a single machine and evolving it into a complex system. 

## 2. Single machine architecture
In contrast to what many people think belief nowadays, in the world of microservices and service-oriented architecture a system composed by only one machine offers many benefits, as well as limitations. 

### Advantages of running a single machine
- Simplicity. Makes up for an easier setup and management, reducing the effort for configuring, maintaing and troubleshooting a single machine. 

- Cost effectiveness. Generally speaking it is indeed cheaper to buy the components for a single machine. Although on the other hand, consumer electronics have become very affordable and powerful. This made distributed computing more affordable as adding more machines not always incured huge bills.

- Data consistency. Data has become the most pivotal piece, everything evolves around the data. Handling data in a single machine is is far more simple than having it distributed in different machines. If we were to simplify this even further and assume there was only one thread, data would always remain consistent and events will always be causal. On the other hand it is easy to foresee this piece to become a huge bottleneck.   

- Performance. There is nothing as performant as running everything locally, data access and information sharing between processes are shared locally, avoiding all possible network delays and latencies that exist in a distributed storage system.  

- Security. A single machine will be less exposed to attacks and the lack of information sharing with other machines makes it impossible to intercept any messages.  

### Limitations and physical problems a single machine may suffer from
- Limited resources (CPU, memory, storage). The main problem is all the resources are shared among all running applications. Therefore a high workload on one app may affect the overall performance, slow response times and lead even to crashes.

- Scalability issues (inability to handle increased workload). This is what happens when the resources of the machine are maxed out. There is no way of scaling, besides vertically. Although consumer electronics have become more affordable and even high end components are not as expensive, there is a limit on how many cores a CPU may have or how much RAM or disk space a machine may have. This can become very expensive and we are not even accounting for downtime for upgrading. 

- Fault tolerance concerns (single point of failure). By far the most important concern when running a single application. Single point of failure. It can happen due to many reasons, including hardware failures, software failures and environmental issues.

    - Hardware failures (disk, CPU, memory, power supply). It is very common for hard drives to fail, RAM sticks to become faulty, or any other component may fail. It is common to configure the disks in RAID for redundancy, have more than one CPU or even to have more than one power supply too. Nonetheless this won't completely guarantee complete prevention from hardware failures. 

    - Software failures (bugs, crashes, memory leaks). There are a myriad of things that can go wrong within a single machine and they are not always intuitive. There could be a systematic error withing the OS, a software bug that causes an application to crash given a certain input, cascading failures where a small fault my trigger a series of consecutive failures. 

    - Environmental issues (power outages, temperature, humidity). Despite hardware redundancy, a machine won't be able to survive a power outage, or even worse, earthquakes or any other unpredictable natural disasters. Temperature and humidity will also greatly affect components lifespan.

## 3. Motivation for distributed fault-tolerant systems
As applications, traffic and specially data grows, the limitations of single machines become more pronounced. There is a need to transition into a system that can provide more resources, better fault tolerance and the ability to scale. We can address these challenges by adding multiple machines which should solve the main pain points of our previous system:

- Fault tolerance, redundancy and single point of failure. In the single machine system, hardware or software failure will automatically lead to system downtime until the fault is resolved. By adding more machines we can solve two painful problems when it comes to failures: the system remains operational even in the presence of a fault, and redundancy, data is not lost and remains available.

- Scalability. By distributing computation and data across different machines it is possible to handle greater workloads than it would ever be in a single machine. Specially in big data where large amounts of data must be processed and analyzed. To overcome peaks of computation or to scale up as the user base grows one can scale the system horizontally, that is, by adding more machines to overcome the demand. Of course this will increase the complexity of the system.

## 4. Introducing multiple machines 
There are initial challenges in coordinating and managing resources. While introducing multiple machines can bring significant benefits, it also presents new challenges. Coordinating and managing resources across multiple machines can be complex, as developers and system admins must deal with issues such as data consistency, load balancing, and fault tolerance. Additionally, networking between machines introduces latency, which can affect application performance. These challenges have a huge repercussion in planning, design, and implementation to ensure that the distributed system operates effectively and efficiently.

In order to measure the design and implementation of a distributed system there are three main properties a successful system should provide: reliability, scalability and maintainability.

- Reliability: the system should keep working, at a desired level of performance, even in the presence of faults and errors.

- Scalability: it should be possible to scale alonside with the system growth in data, traffic or complexity 

- Maintainability: the ability to repair, upgrade or modify the system while it is running without causing downtime (or causing as little as possible)

## 5. Interconnection between machines
The transition from a single machine to a distributed system introduces several complexities, with networking being a critical aspect. Establishing reliable and efficient communication between machines in a distributed system is essential for its overall performance and functionality. 

### Networking basics (LAN, WAN, Internet)
To facilitate communication between machines in a distributed system, they must be interconnected through a network. There are various types of networks to consider:

- Local Area Network (LAN): A LAN connects computers within a limited area, such as an office building or a small campus. LANs enable high-speed communication and data transfer within the confined space, allowing machines within the network to work together efficiently.

- Wide Area Network (WAN): A WAN spans larger geographical areas, connecting machines across cities, countries, or even continents. WANs typically have lower data transfer rates compared to LANs, and they may rely on leased lines or public infrastructure like the Internet.

- Internet: The Internet is a global network that connects countless devices worldwide. Distributed systems can leverage the Internet to enable communication and data exchange between machines in different locations, expanding their reach and capacity.

### Network latency and bandwidth limitations
Distributed systems rely heavily on the performance of the network connecting their machines. There are two key factors, latency and bandwidth:

- Network latency. Refers to the time it takes for data to travel from one machine to another. High latency can (and will most likely) negatively impact the performance of a distributed system by increasing response times and slowing down data transfer. 

- Bandwidth limitations. Represents the maximum data transfer rate of a network. Limited bandwidth will limit the amount of data that can be transferred in a system at a given time, leading to bottlenecks and congestion.

### Communication protocols and data formats
Machines will have to speak, so to say, the same language if they want to understand each other. Distributed systems rely on communication protocols and data formats to exchange information. Common communication protocols include HTTP, gRPC and data formats such as JSON, XML, or Protocol Buffers define the structure of messages exchanged between machines. Choosing appropriate communication protocols and data formats can significantly impact the efficiency, reliability, and maintainability of a distributed system.


## 6. Problems arising from relying on the internet
The internet is often taken for granted as a reliable channel of communication between machines. However, the reality is that the internet is far from a perfect system, and it can be prone to various issues that can negatively impact the performance and functionality of distributed systems.

Some of the problems that can arise from relying on the internet include:

- Unreliable networks: Networks can experience latency, drop connections, or packet loss, causing communication problems between nodes. This can result in delayed or lost messages, leading to coordination issues between nodes.

- Message delays: Messages may be delayed or arrive out of order, causing coordination problems between nodes. This is especially problematic in distributed systems that require strict ordering of messages.

- Unreliable (real) clocks: Clock synchronization is difficult, and nodes may have slightly different clocks, leading to a skewed notion of time. This makes it unreliable to determine causality between events, leading to further coordination issues.

- Node failures: Software and/or hardware issues can cause node failures, which affects the availability and fault tolerance of the system. Nodes are also exposed to malicious attacks, which can further compromise the security and integrity of the system.

- Network partitions: Communication between nodes may be disrupted or compromised, causing partitions that affect the availability and consistency of the system.

- Cascading failures: Failures on one part of the system can cause failures in another, potentially leading to system outages.

All of these issues can have a significant impact on the performance and functionality of distributed systems, nonetheless, it is sometimes hard to design a system taking all possible factors into account. Therefore, when building a system it is important to identify what problems your system capable of tackling and equally important to be aware of what scenarios it is vulnerable to, or simply can't handle. 

In addition to these network-related issues, there are also security concerns to consider. In a distributed system, it's essential to ensure that communication between nodes is secure and that proper authentication and authorization mechanisms are in place to prevent unauthorized access. Encryption is also important to ensure that sensitive data is protected from eavesdroppers and man-in-the-middle attacks.

## 7. Communication methods in distributed systems
There are different communication methods, each with its own set of trade-offs, but it all comes down to two groups, synchronous and asynchronous communication.

First thing that may come to your mind are TCP (Transmission Control Protocol) and UDP (User Datagram Protocol) when talking about async vs sync communications but they must not be confused one for another. TCP and UPD are transport protocols. TCP is a reliable, connection-oriented protocol that ensures data integrity and order of delivery, while UDP is an unreliable, connectionless protocol that provides faster delivery but does not guarantee data integrity nor order of delivery. For this reason, it is safe to assume a UDP connection is always going to imply an asynchronous communication whereas a TCP connection can either be sync or async. The only difference is whether teh sender is going to wait for a response from the receiver or not. In the following example TCP may be used for both async and sync communications.

![Sync/Async replication](/media/async-sync.png)

In the figure above user John Doe makes a request to Service 1. In order to process the user's request Service 1 has to make some computations or replicate data. The communication between Service 1 and Service 2 is synchronous, it will block until it has a confirmation from Service 2 in order to report back to user John Doe. On the other hand, communication with Service 3 is asynchronous, Service 1 sends the request but doesn't wait for the response. 

- Synchronous communication (blocking, immediate response). Simplifies the code structure by following linear execution flow. When a sender makes a request, it waits for the receiver to respond before proceeding. As you can see, a long delay in processing the request from Service 1 to 2 (due to slowness in Service 2) will directly affect the performance or response time to user John Doe. In the worst case scenario, if there is a network fault or the machine running Service 2 crashes it will block the user's request indefinitely. Sometimes you may need this service to be synchronous but can become very dangerous when relying on multiple services or depending on chained services.

- Asynchronous communication (non-blocking, delayed response). Sender makes the request but does not wait for a response. The sender can work on other tasks while the receiver processes the request at its own pace, ultimately responding when ready. This non-blocking approach leads to better resource utilization and improved system performance, as machines are not left idle while waiting for responses. Nonetheless, it introduces more complexities in managing and coordinating tasks. Very popular solution is an event-driven programming to manage execution flow and coordinate tasks effectively - which can make the code more challenging.


## 8. Distributed systems concepts and challenges
To reason about 

- CAP theorem (modelling distributed systems)
- Consistency models (eventual consistency, strong consistency)
- Replication strategies (leader, multi-leader, leader-less replication models)
- Load balancing and resource allocation

## 9. Transactions 
- Properties of transactions (ACID)
- Common problems in transactions (dirty reads, dirty writes, lost update problem, read skew, write skew, phantoms)

## 10. Dealing with failures in distributed systems
- Fault detection and monitoring
- Fault recovery and failover mechanisms
- Data backups and disaster recovery planning

--- 

## Drafts

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

