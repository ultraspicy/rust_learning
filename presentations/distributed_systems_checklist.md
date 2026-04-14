# Distributed Systems Interview Checklist

> Depth levels: **Surface** = know what it is · **Mid** = understand tradeoffs · **Deep** = know where it breaks and why

---

## Consistency & Consensus

- [ ] CAP theorem — and why it's often misapplied
- [ ] PACELC (the better model than CAP)
- [ ] Strong consistency vs eventual consistency vs causal consistency
- [ ] Linearizability vs serializability (commonly confused)
- [ ] Raft — leader election, log replication, commitment rules, minority partition behavior
- [ ] Paxos — at least conceptually
- [ ] Quorum reads/writes (R + W > N)

---

## Replication

- [ ] Single-leader replication
- [ ] Multi-leader replication + conflict resolution strategies
- [ ] Leaderless replication (Dynamo-style)
- [ ] Sync vs async replication tradeoffs
- [ ] Replication lag and read-your-writes guarantee
- [ ] Monotonic reads guarantee

---

## Partitioning / Sharding

- [ ] Range vs hash partitioning
- [ ] Consistent hashing + virtual nodes
- [ ] Hotspot problem and mitigation strategies
- [ ] Rebalancing strategies
- [ ] Secondary indexes across partitions

---

## Transactions

- [ ] ACID properties
- [ ] 2-Phase Commit (2PC) — and why it blocks
- [ ] Distributed transactions vs local transactions
- [ ] Exactly-once, at-least-once, at-most-once semantics
- [ ] Idempotency keys
- [ ] Saga pattern (for long-running distributed transactions)
- [ ] Optimistic vs pessimistic locking

---

## Fault Tolerance

- [ ] Failure modes — crash, Byzantine, network partition
- [ ] Timeouts and retries with exponential backoff + jitter
- [ ] Circuit breaker pattern
- [ ] Bulkhead pattern
- [ ] Heartbeats and failure detection
- [ ] Phi Accrual failure detector

---

## Time & Ordering

- [ ] Why wall clocks are unreliable in distributed systems
- [ ] Logical clocks (Lamport timestamps)
- [ ] Vector clocks
- [ ] Hybrid logical clocks (HLC)
- [ ] Happens-before relationship
- [ ] Total order vs partial order
- [ ] Google TrueTime (Spanner) — bounded clock uncertainty

---

## Storage Internals

- [ ] LSM tree vs B-tree tradeoffs
- [ ] Write-ahead log (WAL)
- [ ] Compaction strategies (leveled vs size-tiered)
- [ ] Bloom filters
- [ ] SSTables and memtables
- [ ] Copy-on-write (MVCC)

---

## Messaging & Streaming

- [ ] Message queue vs event log (Kafka vs RabbitMQ model)
- [ ] Consumer groups and partition assignment
- [ ] Backpressure handling
- [ ] Exactly-once in streaming (transactional producers, idempotent consumers)
- [ ] Ordering guarantees per partition
- [ ] Log compaction
- [ ] Zombie producer fencing (transactional IDs)

---

## Caching

- [ ] Cache invalidation strategies (TTL, event-driven, write-through)
- [ ] Write-through vs write-behind vs write-around
- [ ] Cache stampede / thundering herd + mitigation
- [ ] CDN as distributed cache
- [ ] Cache aside pattern

---

## Distributed Algorithms & Patterns

- [ ] Leader election (beyond Raft — bully algorithm, ZooKeeper)
- [ ] Gossip protocol
- [ ] Merkle trees (for anti-entropy / data sync verification)
- [ ] CRDTs (Conflict-free Replicated Data Types)
- [ ] Two-phase locking (2PL)
- [ ] Sloppy quorum + hinted handoff

---

## Observability

- [ ] The three pillars: metrics, logs, traces
- [ ] Distributed tracing — trace ID propagation across services
- [ ] SLI / SLO / SLA distinctions
- [ ] RED method (Rate, Errors, Duration)
- [ ] USE method (Utilization, Saturation, Errors)

---

## System Design Building Blocks

- [ ] Load balancing strategies (round robin, least connections, consistent hash)
- [ ] Rate limiting algorithms (token bucket, leaky bucket, sliding window)
- [ ] Service discovery (client-side vs server-side)
- [ ] API gateway patterns
- [ ] Backpressure propagation end-to-end
- [ ] Geohash / spatial indexing for location-aware systems

---

## How to Use This Checklist

Unlike coding where a solution is objectively correct, distributed systems interviews reward **reasoning about tradeoffs**. For each topic, practice answering:

1. **What is it?** (Surface)
2. **When would you use it vs alternatives?** (Mid)
3. **Where does it break down and why?** (Deep)

The answer that wins interviews is almost always: *"It depends — here's the tradeoff."*

---

*Based on conversation with Claude · March 2026*
