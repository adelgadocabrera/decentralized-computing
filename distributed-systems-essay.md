# Distributed Systems Essay
> _A distributed system is one in which the failure of a computer you didn't even know existed can render your own computer unusable - Leslie Lamport_

Table of contents:
1. <a href="#section-1">Introduction</a>
2. <a href="#section-2">Single machine architecture</a>
3. <a href="#section-3">Motivation for distributed fault-tolerant systems</a>
4. <a href="#section-4">Introducing multiple machines</a>
5. <a href="#section-5">Interconnection between machines</a>
6. <a href="#section-6">Problems arising from relying on the internet</a>
7. <a href="#section-7">Communication methods in distributed systems</a>
8. <a href="#section-8">Distributed systems concepts and challenges</a>
9. <a href="#section-9">Transactions</a>
10. <a href="#section-10">Consensus</a>
11. <a href="#section-11">Dealing with failures in distributed systems</a>

## 1. <a id="section-1">Introduction</a>

The internet has been around for quite sometime now, and many people have forgotten it wasn't once as reliable as it is now nor are the machines that participate in it. The proposal for this essay is not to cover all the topics related to distributed systems but to shed some light over the complexities that arise in distributed systems, learn why we build such complicated systems, and what problems arise in the process. 

As mentioned, the process is going to follow a bottom-up approach, starting from the simplest form which is running a single machine and evolving it into a complex system. 

## 2. <a id="section-2">Single machine architecture</a>
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

## <a id="section-3">3. Motivation for distributed fault-tolerant systems</a>
As applications, traffic and specially data grows, the limitations of single machines become more pronounced. There is a need to transition into a system that can provide more resources, better fault tolerance and the ability to scale. We can address these challenges by adding multiple machines which should solve the main pain points of our previous system:

- Fault tolerance, redundancy and single point of failure. In the single machine system, hardware or software failure will automatically lead to system downtime until the fault is resolved. By adding more machines we can solve two painful problems when it comes to failures: the system remains operational even in the presence of a fault, and redundancy, data is not lost and remains available.

- Scalability. By distributing computation and data across different machines it is possible to handle greater workloads than it would ever be in a single machine. Specially in big data where large amounts of data must be processed and analyzed. To overcome peaks of computation or to scale up as the user base grows one can scale the system horizontally, that is, by adding more machines to overcome the demand. Of course this will increase the complexity of the system.

## 4. <a id="section-4">Introducing multiple machines</a>
There are initial challenges in coordinating and managing resources. While introducing multiple machines can bring significant benefits, it also presents new challenges. Coordinating and managing resources across multiple machines can be complex, as developers and system admins must deal with issues such as data consistency, load balancing, and fault tolerance. Additionally, networking between machines introduces latency, which can affect application performance. These challenges have a huge repercussion in planning, design, and implementation to ensure that the distributed system operates effectively and efficiently.

In order to measure the design and implementation of a distributed system there are three main properties a successful system should provide: reliability, scalability and maintainability.

- Reliability: the system should keep working, at a desired level of performance, even in the presence of faults and errors.

- Scalability: it should be possible to scale alongside with the system growth in data, traffic or complexity.

- Maintainability: the ability to repair, upgrade or modify the system while it is running without causing downtime (or causing as little as possible)

## 5. <a id="section-5">Interconnection between machines</a>
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


## 6. <a id="section-6">Problems arising from relying on the internet</a>
The internet is often taken for granted as a reliable channel of communication between machines. However, the reality is that the internet is far from a perfect system, and it can be prone to various issues that can negatively impact the performance and functionality of distributed systems.

Some of the problems that can arise from relying on the internet include:

- Unreliable networks: Networks can experience latency, drop connections, or packet loss, causing communication problems between nodes. This can result in delayed or lost messages, leading to coordination issues between nodes.

- Message delays: Messages may be delayed or arrive out of order, causing coordination problems between nodes. This is especially problematic in distributed systems that require strict ordering of messages.

- Unreliable (real) clocks: Clock synchronization is difficult, and nodes may have slightly different clocks, leading to a skewed notion of time. This makes it unreliable to determine causality between events, leading to further coordination issues.

- Node failures: Software and/or hardware issues can cause node failures, which affects the availability and fault tolerance of the system. Nodes are also exposed to malicious attacks, which can further compromise the security and integrity of the system.

- Network partitions: Communication between nodes may be disrupted or compromised, causing partitions that affect the availability and consistency of the system.

- Cascading failures: Failures on one part of the system can cause failures in another, potentially leading to system outages.

All of these issues can have a significant impact on the performance and functionality of distributed systems, nonetheless, it is sometimes hard to design a system taking all possible factors into account. Therefore, when building a system it is important to identify what problems your system is capable of tackling and equally important to be aware of what scenarios it is vulnerable to, or simply can't handle. 

In addition to these network-related issues, there are also security concerns to consider. In a distributed system, it's essential to ensure that communication between nodes is secure and that proper authentication and authorization mechanisms are in place to prevent unauthorized access. Encryption is also important to ensure that sensitive data is protected from eavesdroppers and man-in-the-middle attacks.

## 7. <a id="section-7">Communication methods in distributed systems</a>
There are different communication methods, each with its own set of trade-offs, but it all comes down to two groups, synchronous and asynchronous communication.

First thing that may come to your mind are TCP (Transmission Control Protocol) and UDP (User Datagram Protocol) when talking about async vs sync communications but they must not be confused one for another. TCP and UPD are transport protocols. TCP is a reliable, connection-oriented protocol that ensures data integrity and order of delivery, while UDP is an unreliable, connectionless protocol that provides faster delivery but does not guarantee data integrity nor order of delivery. For this reason, it is safe to assume a UDP connection is always going to imply an asynchronous communication whereas a TCP connection can either be sync or async. The only difference is whether the sender is going to wait for a response from the receiver or not. In the following example TCP may be used for both async and sync communications.

![Sync/Async replication](/media/async-sync.png)
<p align="center">
  <img src="/media/async-sync.png" width="300" style="margin-bottom: 50px;">
</p>

In the figure above user John Doe makes a request to Service 1. In order to process the user's request Service 1 has to make some computations or replicate data. The communication between Service 1 and Service 2 is synchronous, it will block until it has a confirmation from Service 2 in order to report back to user John Doe. On the other hand, communication with Service 3 is asynchronous, Service 1 sends the request but doesn't wait for the response. 

- Synchronous communication (blocking, immediate response). Simplifies the code structure by following linear execution flow. When a sender makes a request, it waits for the receiver to respond before proceeding. As you can see, a long delay in processing the request from Service 1 to 2 (due to slowness in Service 2) will directly affect the performance or response time to user John Doe. In the worst case scenario, if there is a network fault or the machine running Service 2 crashes it will block the user's request indefinitely. Sometimes you may need this service to be synchronous but can become very dangerous when relying on multiple services or depending on chained services.

- Asynchronous communication (non-blocking, delayed response). Sender makes the request but does not wait for a response. The sender can work on other tasks while the receiver processes the request at its own pace, ultimately responding when ready. This non-blocking approach leads to better resource utilization and improved system performance, as machines are not left idle while waiting for responses. Nonetheless, it introduces more complexities in managing and coordinating tasks. Very popular solution is an event-driven programming to manage execution flow and coordinate tasks effectively - which can make the code more challenging.


## 8. <a id="section-8">Distributed systems concepts and challenges</a>
In this section, we will explore several fundamental concepts and challenges related to distributed systems. We will delve into the expected behavior of a distributed system, the CAP theorem, which helps model the trade-offs between consistency, availability, and partition tolerance. Consistency models, such as eventual consistency and strong consistency, define how data is synchronized and shared among distributed components. We will also examine different replication strategies, including leader-based, multi-leader, and leader-less models, which determine how data is replicated across multiple machines.

### Liveness and safety
Liveness and safety are two fundamental properties used to reason about the correctness and behavior of distributed systems.

- Liveness: Liveness refers to the property that a system will eventually make progress and produce a desired outcome. It ensures that something good eventually happens in the system. In other words, liveness guarantees that a system will not get "stuck" or remain in an undesirable state indefinitely. Liveness properties are crucial to ensure that distributed systems continue to operate and progress towards their intended goals, even in the presence of failures or other challenges. For example:
    - Every order placed by a customer is eventually processed and fulfilled, and every payment made by a customer is eventually confirmed.

- Safety: Safety refers to the property that a system always maintains certain desired invariants or properties, regardless of its past or future behavior. It ensures that something bad never happens in the system. Safety properties express constraints on the system's behavior and specify what should never occur. For example:
    - The system ensures that a customer cannot place an order without sufficient funds in their account, preventing invalid or unauthorized orders. Additionally, the system guarantees that a customer is charged only once for a given order, preventing duplicate payments.

### CAP theorem (modelling distributed systems)
The CAP theorem, also known as Brewer's theorem, is a fundamental concept in distributed systems that helps in understanding the trade-offs between three desirable properties: consistency, availability, and partition tolerance. 

![CAP Theorem](/media/cap.png)

- Consistency: Consistency refers to the requirement that all nodes in a distributed system have the same view of the data at any given time. In other words, when a client reads data from one node, any subsequent reads from other nodes should return the same value or a consistent state. Strong consistency models, such as linearizability, provide globally consistent results. However, achieving strong consistency may come at the cost of availability or increased latency in the face of network partitions or failures.

- Availability: Availability means that every request made to a non-failing node in the system should receive a response, regardless of the state of other nodes. In other words, the system should remain operational and continue to serve requests even in the presence of failures. High availability is crucial for systems that require continuous operation and cannot afford significant downtime. However, ensuring availability might lead to relaxed consistency guarantees or potential data staleness in certain scenarios.

- Partition Tolerance: Partition tolerance refers to the system's ability to continue operating even when network partitions occur, causing nodes to be isolated and unable to communicate with each other. Network partitions can be caused by network failures or delays. Distributed systems must be able to tolerate and handle such partitions to ensure system availability and prevent a complete system failure. However, partition tolerance may require sacrificing either consistency or availability under certain conditions.

According to the CAP theorem, a distributed system can only guarantee two out of the three properties: consistency, availability, and partition tolerance. This means that in the presence of network partitions, system designers must choose whether to prioritize consistency (CP systems) or availability (AP systems).

### Consistency models 
Consistency models define how data is synchronized and shared among distributed components in a distributed system. They provide guidelines for how updates and reads are performed and how the system ensures data integrity. Different consistency models offer varying levels of guarantees regarding the visibility of updates and the ordering of operations. Let's take a look at some consistency models.

- Eventual Consistency: Eventual consistency allows replicas to become consistent over time without enforcing strict ordering or immediate visibility of updates. It permits temporary inconsistencies but guarantees that all replicas will _eventually_ converge to the same state. Systems employing eventual consistency prioritize availability and low latency, making it suitable for applications such as content delivery networks (CDNs), distributed databases, and collaborative editing systems.

- Strong Consistency (Linearizability): Strong consistency provides the _strongest_ level of consistency guarantees. It ensures that all operations appear to take effect atomically and in a globally agreed-upon order. In other words, operations appear as if they were executed in a single machine. With strong consistency, all replicas observe the same order of operations, and any read operation will return the most recent write or a concurrent write's result. Strong consistency is critical in applications that require strict data integrity, such as banking systems or e-commerce platforms.

- Causal Consistency: Causal consistency guarantees that if one operation _causally_ depends on another, all replicas will observe them in the same order. It preserves causality between related operations while allowing some temporary inconsistencies in unrelated operations. Causal consistency is useful in systems that require preserving causal relationships between events, such as collaborative applications or event-driven architectures.

- Session Consistency: Session consistency guarantees that all operations within a _single client session_ appear to take effect in the order specified by the client. It provides consistency within the context of a specific session while allowing temporary inconsistencies across sessions. Session consistency is suitable for scenarios where maintaining a consistent view within a session is important, such as in e-commerce shopping carts or online collaborative sessions.

- Read-your-Writes Consistency: Read-your-writes consistency ensures that any read operation performed by a client will always return the result of its own preceding write operation. It guarantees that a client observes its own writes immediately. Read-your-writes consistency is valuable in applications where immediate visibility of updates is necessary for correctness, such as social media feeds or real-time collaborative editing. One caveat though. How do you ensure read-your-writes if you try to read your own post from a different device? In this case it may appear inconsistent. 

### Replication strategies 
Replication strategies are essential in distributed systems to achieve fault tolerance, improve availability, and enhance performance. Replication involves maintaining multiple copies of data across distributed components. Various replication strategies exist, each offering different trade-offs in terms of consistency, latency, and system complexity. Let's explore some common replication strategies:

- Leader-based Replication
    - In leader-based replication, one replica, known as the leader or primary replica, is designated as the primary copy for all write operations. When a client wants to write data, it sends the write request to the leader replica, which processes and propagates the update to the other replicas, known as followers or secondary replicas. Read operations can be performed on either the leader or the followers, increasing (read) availability.
    - Advantages of leader-based replication include low write latency, as write operations only need to be processed by the leader, and strong consistency guarantees as long as clients always read from the leader. However, if the leader fails, a new leader must be elected, and there might be a delay during which the system is unavailable for writes. Additionally, the leader can become a bottleneck for write-intensive workloads.

- Multi-leader Replication
    - In multi-leader replication, multiple replicas are designated as leaders, each responsible for handling write operations for a specific subset of data. Clients can write to any of the leaders, and updates are asynchronously propagated to other replicas. Read operations can be performed on any replica, but conflicts may arise due to concurrent writes on different leaders.
    - Multi-leader replication provides high availability, as write operations can be processed by any leader replica, reducing write latency and enabling better scalability. However, managing conflicts and achieving consistency across leaders becomes a challenge. Conflict resolution mechanisms and techniques such as conflict detection, resolution policies, or application-level conflict handling must be employed to maintain data integrity.

- Leader-less Replication 
    - In leader-less replication, also known as distributed consistency models, there is no designated leader or primary replica. Instead, all replicas participate in the coordination and decision-making process. Write operations are typically performed on multiple replicas, and the system employs consensus protocols or voting mechanisms to ensure that updates are applied consistently across replicas. Read operations can be performed on any replica.
    - Leader-less replication provides high availability, fault tolerance, and load balancing across replicas. It eliminates the single point of failure and bottleneck associated with a leader-based approach. However, achieving consensus among replicas can introduce higher write latency, increased communication overhead, and potential conflicts that need to be resolved.

- Hybrid Replication
    - Hybrid replication combines multiple replication strategies to leverage the advantages of different approaches based on the specific needs of the application or data. For example, a system might use leader-based replication within a data center for low-latency writes and employ multi-leader replication across multiple data centers for geo-distribution and fault tolerance.
    - Hybrid replication allows system designers to tailor the replication strategy to the specific requirements of their application, optimizing for factors such as consistency, availability, latency, and scalability.

Example of hybrid replication. Each data center has a leader accepting writes. Writes are propagated to other data centers and replicated within the same data center using sync and async communication.
![Hybrid replication](/media/replication.png)

## 9. <a id="section-9">Transactions</a>
Transactions are a fundamental unit of work that consists of a set of operations that should be executed atomically and consistently. They provide a way to group multiple operations into a single logical unit, ensuring that either all operations within the transaction are successfully completed, or none of them take effect at all. By providing this atomicity property, transactions help to preserve data integrity and prevent inconsistent or partial updates.

### Properties of transactions (ACID)
ACID is an acronym that stands for Atomicity, Consistency, Isolation, and Durability.

- Atomicity: Atomicity ensures that a transaction is treated as an indivisible unit of work. It guarantees that either all operations within a transaction are successfully completed and committed, or none of them take effect at all. If any part of a transaction fails, the entire transaction is rolled back, and the system remains in its original state. Atomicity guarantees that transactions are either fully completed or fully undone, preventing partial updates and preserving data integrity. An alternative name could be _abortability_ as atomicity could be confused for atomicity in multi-threading.

- Consistency: Consistency ensures that a transaction brings the system from one consistent state to another. It defines the integrity constraints and rules that the data must adhere to. Transactions must follow predefined consistency rules, ensuring that the system's data remains valid and satisfies all constraints, business rules, and integrity requirements. Consistency guarantees that only valid and permissible changes are made to the data, preserving its overall correctness. These rules are the so called business rules and thus consistency is not really a property of a transaction from the point of view of a database's responsibility.

- Isolation: Isolation guarantees that each transaction is executed in isolation from other concurrently executing transactions. It ensures that the intermediate states of a transaction are not visible to other transactions until it is committed. Isolation prevents interference, conflicts, and inconsistencies that can arise when multiple transactions concurrently access and modify the same data. It provides the illusion that transactions are executed sequentially, even though they may be executing concurrently.

- Durability: Durability ensures that once a transaction is committed, its effects are permanently recorded and will persist even in the event of system failures, crashes, or power outages. Committed data is stored in a durable and permanent manner, typically in non-volatile storage, such as disk or flash memory. Durability guarantees that data changes made by committed transactions are persistent and can be recovered in the face of failures.

### Common problems in transactions 
Here are common problems that can occur in transactions:

- Dirty Reads: A dirty read occurs when one transaction reads data that has been modified by another transaction that has not yet committed. This can lead to inconsistent or incorrect data being read. 

- Dirty Writes: Dirty writes happen when one transaction overwrites data that has been modified by another transaction that has not yet committed. This can result in data loss or inconsistency.

- Lost Updates: Lost updates occur when two transactions concurrently read the same data and then update it independently, resulting in one update being lost, and the final state not reflecting both changes.

- Read Skew: Read skew refers to a situation where a transaction reads data twice but obtains different results due to concurrent updates by other transactions. This can lead to inconsistencies when making decisions based on the read data.

- Write Skew: Write skew occurs when two transactions read some overlapping data, make independent updates based on that data, and commit their changes concurrently. This can lead to conflicts and inconsistent states.

- Phantoms: Phantoms refer to situations where a transaction performs a query multiple times, and the results differ due to concurrent insertions or deletions by other transactions. This can lead to unexpected and inconsistent query results.

### Isolation levels 
There are several commonly used isolation levels that define the degree of concurrency and consistency in executing transactions. Each isolation level offers different guarantees and trade-offs regarding data visibility, concurrency control, and isolation from other concurrent transactions.

- Read Uncommitted is the lowest isolation level, where transactions can read data that has been modified but not yet committed by other concurrent transactions. This level provides the least consistency and exposes transactions to phenomena like _dirty reads_, _dirty writes_, and _non-repeatable reads_. It offers high concurrency but sacrifices data integrity.

- Read Committed guarantees that a transaction will only read data that has been committed by other transactions. It avoids dirty reads but allows non-repeatable reads, as data can be modified by concurrent transactions during the course of a transaction. Read Committed provides a higher level of consistency compared to Read Uncommitted.

- Repeatable Read ensures that within a transaction, the same query will always return the same set of rows, even if other transactions modify the data concurrently. It prevents non-repeatable reads by acquiring shared locks on read data, preventing other transactions from modifying it until the transaction completes. However, it still allows phantom reads, where new rows may be inserted by other transactions.

- Serializable is the highest isolation level, providing the strongest consistency guarantees. It ensures that transactions execute as if they were executed one after another, in a serial manner. Serializable prevents all concurrency-related anomalies, including dirty reads, non-repeatable reads, and phantom reads. It achieves this by acquiring exclusive locks on read data, preventing other transactions from accessing or modifying it until the transaction completes. Serializable offers the strongest data integrity but can impact concurrency and scalability due to increased locking.

### Concurrency Control Mechanisms 
Different isolation levels can be achieved through various concurrency control mechanisms and techniques. Each mechanism provides a different balance between data consistency and concurrency. 

- Serial Execution
    - Serial Execution: Serial Execution guarantees the highest level of isolation and consistency by executing transactions sequentially, one after another. Serial execution ensures that there are no concurrency-related anomalies, but it severely limits concurrency and may lead to decreased performance.
    
- Optimistic Concurrency Control (OCC)
    - Optimistic Concurrency Control (OCC): OCC allows multiple transactions to proceed concurrently without acquiring locks. Each transaction performs its operations and validates them during the commit phase to ensure that no conflicts occurred. OCC offers high concurrency but may result in increased rollback rates if conflicts are detected during validation.

- Multiversion Concurrency Control (MVCC)
    - Snapshot Isolation: Snapshot Isolation allows each transaction to operate on a consistent snapshot of the database taken at the start of the transaction. It achieves a level of isolation similar to Repeatable Read. MVCC provides good concurrency by allowing read and write operations to proceed concurrently, but it can result in increased storage requirements due to maintaining multiple versions of data.

- Timestamp-Based Concurrency Control
    - Serializable Snapshot Isolation (SSI): SSI uses timestamps to order transactions and determine the visibility of data. It achieves Serializable isolation by preventing phenomena such as dirty reads, non-repeatable reads, and phantom reads. However, SSI may introduce transaction aborts and serialization anomalies in highly concurrent workloads.

- Lock-Based Concurrency Control
    - Two-Phase Locking (2PL): Two-Phase Locking is a widely used mechanism for achieving isolation levels such as Repeatable Read and Serializable. It involves two phases: a growing phase where locks are acquired, and a shrinking phase where locks are released. Two-Phase Locking provides strong isolation guarantees but can lead to lock contention and reduced concurrency, potentially impacting performance.

It's important to consider the trade-offs associated with each mechanism. Lock-based mechanisms provide strong isolation but can impact concurrency and may introduce lock contention. MVCC and timestamp-based mechanisms offer better concurrency but require additional storage and can have overhead in detecting conflicts. Optimistic Concurrency Control optimistically assumes there are no conflicts, but it incurs the cost of potential rollbacks. Serial execution guarantees consistency but significantly limits concurrency.

The choice of mechanism depends on the specific requirements of the application, the workload characteristics, and the desired balance between consistency and concurrency. It's essential to carefully evaluate the trade-offs and choose the most suitable mechanism to achieve the desired isolation level while ensuring optimal system performance.


## 10. <a id="section-10">Consensus</a>
Consensus algorithms are essential in distributed systems, whether they are private systems with machines belonging to a single owner or decentralized systems involving multiple independent entities. The primary purpose of consensus algorithms is to enable these distributed systems to reach an agreement among multiple nodes or processes on a common decision or state, even in the presence of failures or network partitions.

In distributed systems, individual nodes operate independently and may have different views of the system or its data. Achieving consensus ensures that all nodes converge to a consistent state and agree on important aspects such as the order of operations or the value of a particular data item. Consensus is crucial for maintaining system integrity, ensuring data consistency, and enabling coordinated actions across multiple nodes.

In private distributed systems, consensus algorithms provide a means for nodes belonging to a single owner to agree on critical decisions. For example, in a cluster of servers, consensus algorithms can be used to elect a leader node, determine the order of data updates, or agree on a common configuration. By reaching consensus, private systems can operate cohesively and avoid conflicts or inconsistent states that could arise due to independent decision-making by individual nodes.

On the other hand, consensus algorithms play a crucial role in decentralized systems, where multiple independent entities participate in a distributed network. These entities may have different owners, objectives, and levels of trust. Consensus allows these entities to work together, establish agreement, and enable decentralized coordination without relying on a central authority. Examples of decentralized systems include blockchain networks, peer-to-peer networks, and distributed ledgers.

Decentralized systems often face challenges such as malicious actors, Byzantine failures, or the absence of a trusted central authority. Consensus algorithms address these challenges by providing mechanisms to validate and agree on the correctness of transactions, prevent double-spending, or ensure the consistency of the shared ledger. Through consensus, decentralized systems achieve trust, transparency, and immutability, enabling applications like cryptocurrencies, smart contracts, and distributed databases.

Some common examples of scenarios where consensus algorithms are commonly used:

- Distributed Databases: In a distributed database system, multiple nodes store and manage data across different locations. Consensus algorithms are used to ensure that all nodes agree on the order and consistency of data updates. This enables data replication, fault tolerance, and ensures that queries return consistent results across all nodes.

- Blockchain Networks: Blockchain networks are decentralized systems where multiple nodes participate in the validation and agreement of transactions. Consensus algorithms, such as Proof of Work (PoW) or Proof of Stake (PoS), are used to achieve agreement on the order and validity of transactions, as well as to prevent malicious activities like double-spending.

- Distributed File Systems: In distributed file systems, files are stored across multiple nodes for fault tolerance and improved performance. Consensus algorithms help ensure that all nodes agree on the availability, integrity, and consistency of files. This allows for reliable data storage and retrieval in the face of failures or network partitions.

- Decentralized Governance: In decentralized governance models, consensus algorithms play a crucial role in decision-making and voting processes. They allow participants to agree on proposals, rule changes, or the election of representatives in a transparent and secure manner.


## 10. <a id="section-11">Dealing with failures in distributed systems</a>
Fault tolerance is a key aspect of dealing with failures in distributed systems. It refers to the ability of a system to continue operating and providing its services in the presence of failures. Fault tolerance mechanisms aim to minimize the impact of failures, ensure system availability, and maintain the desired level of performance and functionality.

Some strategies may include:

- Redundancy: Redundancy involves creating duplicate components, data, or resources to provide backups. By maintaining redundant copies, the system can continue functioning even if some components fail. For example, using drives in RAID mode in case one of the drives dies. Redundancy enhances fault tolerance and allows for seamless failover when failures occur.

- Replication: Replication is the process of creating multiple copies of data or system components across different nodes as we have seen before. Replication improves system availability and reliability by allowing requests to be processed by replicas if the primary node fails. It ensures liveness by enabling continuous operation and safety by preserving data integrity.

- Failover: Failover refers to the process of transferring operations from a failed node to a backup component. When a failure is detected, the system automatically switches to an alternative node to continue processing requests. Failover mechanisms ensure continuity and minimize downtime by quickly recovering from failures.

### Fault Detection and Monitoring
Fault detection involves identifying abnormal behavior, errors, or failures in the system components. This can be achieved through various monitoring techniques, including:

- Health Checks: Regularly monitoring the health and status of individual components or nodes in the system. This can involve checking metrics such as CPU usage, memory utilization, network connectivity, and response times.

- Heartbeats and Ping Messages: Using heartbeat signals or periodic ping messages to check the availability and responsiveness of components. If a heartbeat or ping message is not received within a specified timeframe, it indicates a potential failure.

- Distributed Tracing: Employing distributed tracing techniques to trace and monitor the flow of requests and responses across the distributed system. This can help identify performance bottlenecks, failures, or latency issues.

- Log Analysis: Analyzing system logs to detect anomalies, errors, or patterns that indicate potential failures or issues in the system.
