# I Can’t Believe It’s Not Causal! Scalable Causal
Consistency with No Slowdown Cascades

## Problem Statement
- Causal Consistency - Produce Slowdown Cascade 
	- Justin writes W<sub>1</sub> 
	- Alice reads W<sub>1</sub> 
	- Alice writes W<sub>2</sub> 
	- Alice writes W<sub>3</sub> 
	- If some user with different locality tries to access W<sub>3</sub> directly, it will have to wait for W<sub>2</sub> which at the same time will have to wait for W<sub>1</sub> for causality. 
	- _What if orange replica (top right) is slow?_
	- Slowdown Cascade!
	- **On top of that**, Alice's W<sub>2</sub>, which is a PDF upload, has no relationship with W<sub>1</sub> as W<sub>1</sub> could be just some random comment from Justin
	- _Why wait on replicated W<sub>1</sub>?_
	- Solution? **OCCULT**

## OCCULT 
**Observable Causal Consistency Using Lossy Timestamps**

- Observable Causal Consistency 
	- Reminder of Causal Consistency  
    - Causal Consistency guarantees that each **client observes** a monotonically non-decreasing set of updates (including its own) in an order that respects potential causality between operations.

### Observable Consistency Key Idea
- Don't implement a causally consistent data store 
- Let clients observe a causally consistent data store 

### Features and data flow 
- Writes are accepted at Master Shards and are replicated (without any waiting) asynchronously but in order to slaves
- Each shard (master or slave) keeps track of a **shardstamp**
- **Shardstamp** counts the writes applied to the shard
- Client metadata == vector of **shardstamps** which identifies a global state across all shards
- Vector of **shardstamps** == **Causal Timestamp**
- On replication causal timestamps are also sent
- Causal timestamp is the number of writes a client knows of each shard

#### Example (slide 18)
1. Client 1 with causal timestamp [4,3,2] writes w(a) to Master with shardstamp 7 
2. Master updates shardstamp to 8 
3. Master updates clients causal timestamp to [8,3,2] 
4. Client 2 with causal timestamp [6,2,5] reads from Master with now causal timestamp [8,3,2] 
5. Client 2 updates causal timestamp to [8,3,5] 
6. and so on (check slides)

### Problem 
Keeping track of causal timestamps for **hundreds of thousands of nodes**.

### Causal Timestamp Compression
#### Structural Compression 
Make causal timestamp (shardstamp vector) smaller by locating each sharstamp at the modulo of $n$ entry. This reduces vector size $N$ to $n$. But now when reading from a replica, if an entry may have a larger value than one of the replicas at that entry, values will be stale and client will have to wait. Solution: 
- Use system clock as the next value of shardstamp on a write 
- Decouples shardstamp value from number of writes on each shard
- _This helps, but system is still relying on reducing number of entries by using modulo which is not ideal_

#### Temporal Compression 
- Real clocks can reduce spurious dependencies but not solve the limitation of using modulo arithmetic to compress causal timestamps.
- Recent shardstamps are more likely to generate spurious dependencies, so temporal compression focuses a disproportionate fraction of its ability on the shards with the most recent shardstamps.
- All other shards are mapped to the vector's catch-all last entry, which will reflect updates that were accepted a while ago.
- When a client tries to read from a conflated shard, it is likely that the shardstamp of that shard has already exceeded the value stored in the catch-all entry.

#### Isolating Datacenters
- Loosely synchronized timestamps used to curb spurious dependencies in datacenters can be affected by replication delays and clock skew.
- To solve this, distinct causal timestamps are used for each datacenter to mitigate stale reads.
- If the clocks in one datacenter run ahead of another datacenter, the window of inconsistency for clients in the first datacenter can be much longer than for clients in the second datacenter, potentially leading to more stale reads.
- The additional overhead is reduced since the number of causal timestamps does not grow with the number of datacenters, and these timestamps need fewer entries due to close synchronization within each datacenter.

### Transactions 
1. **Observable** Atomicity 
2. **Observably** read from a casually consistent snapshot 
3. No concurrent conflicting writes 

#### Properties of Protocol
1. No centralized timestamp authorities (e.g. per-datacenter)
	1. Transactions ordered using causal timestamps 
2. Transaction commit latency is independent of the number of replicas - replication async after committed on master

#### Three Phase Protocol 
For a client $c$ performing a write $w$ in a transaction $T$: 

1. **Read Phase** 
	- $c$ obtains from the appropriate shards the objects $o$ that $T$ reads 
	- (making sure they're not stale values)
	- and $c$ locally buffers $T$'s writes 
2. **Validation Phase**
	- $c$ ensures that all read objects belong to a consistent snapshot using causal timestamps 
	- $c$ locks every object part of a write in $T$ by contacting each shard s<sub>o</sub>  
	- (Concurrent reads are allowed in the meantime)
	- If $c$ doesn't have exclusive write access, $c$ must restart this phase
	- Upon locking shard master(s) responds with
		1. $o$'s causal timestamp  
		2. New shardstamp that will be assigned to $w$ 
1. **Commit Phase**
	- $c$ computes $T$'s commit timestamp using prev shardstamps
	- $c$ buffered writes committed in an observably atomic way all objects updated by $T$ 
	- $c$ updates its own causal timestamp 

This process guarantees atomic transactions even though replication happens async.

## Questions 
- If only one master per shard(s), means (many) users far away from master shard will see slow writes? How do they tackle this?
- For a given write w, master shard first updates shardstamp and then persists w. Is this w stored with the causal timestamp at the given time it was committed as metadata? Or does the master shard only keep track of the most recent causal timestamp? From transactions explanation sounds like every $w$ has a causal timestamp attached.
- It says that if client tries to read latest updates from replica it compares the shardstamp, if client's shardstamp is greater it means replica hasn't received latest updates. Talks about mechanism for waiting. Its not exactly the same as slowdown cascades, but isn't it kind of similar in a way? At least there could be huge slowdowns in this case. 
- I don't fully understand the naive compression mechanism. They map shards by mapping them dividing modulo n to the same entry. What does that even mean? If two shards map to entry 1, you store the greatest value of the two? From the example given in the YT presentation that seems to be the case.
- **Transactions Validation Phase** why don't they define what $i$ and $j$ are?
	- "**Validation phase** Validation involves three steps. In the first, $c$ verifies that the objects in its read set belong to a consistent snapshot Σ<sub>rs</sub>. It does so by checking that all pairs o<sub>i</sub> and o<sub>j</sub> of such objects are pairwise consistent [12], i.e., that the saved shardstamp of the shard s<sub>o<sub>i</sub></sub> from which o<sub>i</sub> was read is at least as up to date as the entry for s<sub>o<sub>i</sub></sub> in the causal timestamp of o<sub>j</sub> (and vice versa)."

