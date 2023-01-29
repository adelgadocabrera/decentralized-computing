# Google Spanner 
Google Spanner is a "**scalable**, **multi-version**, **globally-distributed**, and **synchronously-replicated** **database**" designed, built and deployed at Google. It is designed to provide high availability and strong consistency - it is actually the first externally consistent distributed database at global scale.

Expanding on the Google Spanner definition:
- **Scalable** - designed to scale horizontally
	- A Spanner deployment is called a _universe_
	- Spanner scales out by adding or removing _zones_, the smallest unit of deployment 
	- A zone has a _zonemaster_, thousands of _spanservers_ and _location proxies_
		- **zonemasters** assign data to spanservers
		- **spanservers** serve data to the clients
		- **location proxies** are used by clients to locate data in spanservers
- **Multi-version** - commits are automatically timestamped, every value has attached said timestamp instead of overwriting previous value. This grants the system serializable transaction isolation and linearizable reads and writes. This technique of using timestamps called Multi-Version Concurrency Control (MVCC), is already used by other databases like PostgreSQL. 
	- Because you can discriminate commits up to a certain timestamp (**consistent snapshots**), it allows for lock-less read-only transactions
		- Requires negotiation phase between all Paxos groups that are involved in the reads though
	- This increases performance and throughput as you don't have to acquire locks and potentially stall thousands of requests, for example, creating a back up
- **Globally-distributed** - Google Spanner is designed to be deployed across millions of machines globally (with petabytes of data). Data is sharded between thousands of machines across different continents, and even further, replicated to various data-centers
- **Database** - semi-relational database to store data with ACID guarantees. 
	- The data structure stored is called _tablets_, stored in B-trees, in write-ahead logs stored in Colossus.

Some additional techniques used:
- **Paxos** - state machine replication within a shard
	- Writes initiate the Paxos protocol at the leader
	- Reads access state directly from _tablets_ if replica is up-to-date
	- Data is moved across Paxos groups _directory_ by _directory_ - abstraction based on contiguous keys
		- Allows for clients to describe data locality relationship 
- **Two-Phase Locking** for serializability
- **Two-Phase Commit** for cross-shard atomicity
- **TrueTime** - its an API that exposes clock uncertainty. There are machines in every data-center with GPS receivers and atomic clocks with the sole purpose of keeping time synchronization between nodes
	- Why so crucial? 
		- Physical clocks are not reliable, two recorded events have no guarantee of causality - clock skew, clock drift
		- Logical clocks do provide causality, but linearizability relies on real-time order
		- TrueTime solves these problems with an API that provides an interval instead, from earliest possible timestamp to latest
		- Reduces requests between nodes as all agree on TrueTime API


