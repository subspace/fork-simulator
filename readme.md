## Fork Simulator

A simple simulator to measure the expected fork rate of different constructions of the Subspace protocol.

### Workflow

1. Plotting Pieces

* Starting with a deterministic plot seed
* Iterate a nonce and hash that with seed to create a unique encoded hash
* Take the first 8 bytes
* Add that to the S-Tree with with <hash, index>

2. Solving the Challenge

* Starting with a deterministic challenge seed (the genesis block hash)
* Query the BST with the first 8 bytes of the block hash 
* Find all solutions with the three best quality steps (across the network)
* For each solution, determine the time delay 
* Set a timeout for the time delay
* When the timeout expires, add a new block to the DAG (hash, quality, parent)
* Call the solve function again, using the new block as the seed

1. Tracking Chain Growth

* Need a data structure to represent the ledger -- some kind of DAG
* Need an array of all the heads of the DAG and the quality of each head
* Every time a chain is extended we will extend the current head with the highest quality branch
* Each other branch will form a new head pointing to the parent
* At some interval (every second) output a list of all heads

## Protocol Requirements

1. Converge (honest network will see a single chain)
2. Secure (51% storage)
3. Fair (no cpu advantage)
4. Useful (sybil-resistant) -> Multiple Proofs
5. Efficient 

## Developing a Secure Construction

1. Proof-of-Replication -> PoR-128 (each encoding)
2. Proof-of-Storage -> Prove that we have plotted many pieces (an audit)

1. How do we have a protocol that converges to a single chain, assuming all nodes are honest? -> exponential quality 
2. How do we prevent an adversary from mining a private chain faster than the honest network? -> Proof of Time
   1. Farm many weak blocks faster than the honest network grows a strong chain
   2. Mine a private chain using CPU power alone (not feasible)
3. Fair -> 250x advantage with compression and encoding on demand, just chain the encodings together 
4. Sybil Resistant / Load Balanced -> all encodings must be under the same node id and have a minimum distance, with bonus for distance
5. 