# Technical Terms Glossary

**Purpose**: Comprehensive definitions of all technical terms used in the Offline NPoS Election Tool documentation.

## Algorithm Terms

### NPoS (Nominated Proof of Stake)

**Definition**: A consensus mechanism used by Polkadot and many Substrate-based chains to select validators. In NPoS, validators are selected from a pool of candidates based on stake backing from nominators.

**Key Characteristics**:
- Validators produce blocks and validate transactions
- Nominators stake tokens to back validators
- Elections run periodically to select validators
- Stake distribution determines selection probability

**Related Terms**: Validator, Nominator, Active Set, Stake

### Sequential Phragmen

**Definition**: A standard election algorithm that selects validators sequentially based on stake distribution. Uses `sp_npos_elections::seq_phragmen` from Substrate.

**Key Characteristics**:
- Deterministic algorithm
- Standard algorithm used by most Substrate chains
- Produces consistent results
- Selects validators one at a time based on stake backing

**Related Terms**: Parallel Phragmen, Multi-phase, Election Algorithm

### Parallel Phragmen

**Definition**: An alternative election algorithm that can produce different results than sequential phragmen. Uses `sp_npos_elections::phragmms` (Phragmms algorithm) from Substrate.

**Key Characteristics**:
- Alternative to sequential phragmen
- Can produce different validator selections
- Useful for comparing outcomes with sequential phragmen
- Uses parallel computation approach

**Related Terms**: Sequential Phragmen, Multi-phase, Election Algorithm

### Multi-phase

**Definition**: A multi-phase election process with signed/unsigned submissions and fallback phases. Uses sequential phragmen internally, matching `pallet-election-provider-multi-phase`.

**Key Characteristics**:
- Multiple phases: signed submissions, unsigned submissions, fallback
- Uses sequential phragmen as underlying algorithm
- Represents the multi-phase election process used by chains like Polkadot
- Supports complex election scenarios

**Related Terms**: Sequential Phragmen, Election Algorithm, Phragmen

### Phragmen

**Definition**: A family of election algorithms named after Swedish mathematician Lars Edvard Phragmén. Phragmen algorithms are used for proportional representation and validator selection in blockchain networks.

**Key Characteristics**:
- Named after Lars Edvard Phragmén
- Used for proportional representation
- Multiple variants: sequential, parallel, etc.
- Foundation for NPoS election algorithms

**Related Terms**: Sequential Phragmen, Parallel Phragmen, Election Algorithm

## Polkadot Ecosystem Terms

### Validator

**Definition**: A node that participates in the consensus mechanism of the Polkadot network. Validators are responsible for producing blocks, finalizing blocks, validating parachain blocks, and participating in governance.

**Key Characteristics**:
- Produces blocks on the relay chain
- Finalizes blocks through consensus
- Validates parachain blocks
- Participates in network governance
- Earns rewards for participation

**Related Terms**: Nominator, Active Set, Stake, NPoS

### Nominator

**Definition**: A token holder who stakes tokens by nominating validators. Nominators don't run validator nodes themselves but support validators they trust.

**Key Characteristics**:
- Stakes tokens to back validators
- Selects validators to nominate
- Earns rewards when validators perform well
- Shares rewards with validators (validators take commission)
- Monitors validator performance

**Related Terms**: Validator, Stake, NPoS, Active Set

### Stake

**Definition**: Tokens locked to participate in the network's security and consensus mechanism. In Polkadot, validators stake to become candidates, and nominators stake to back validators.

**Key Characteristics**:
- Tokens are bonded (locked) when staking
- Staking is not permanent (can be unbonded)
- Slashing can occur if validators misbehave
- Rewards are distributed automatically
- Determines validator selection probability

**Related Terms**: Validator, Nominator, Active Set, NPoS

### Active Set

**Definition**: The fixed number of validators selected to participate in consensus. For Polkadot mainnet, the active set is typically around 297 validators (as of 2025). The size can change through governance.

**Key Characteristics**:
- Fixed number of validators
- Selected through NPoS elections
- Participates in consensus and block production
- Size can change through governance
- Rotates periodically through elections

**Related Terms**: Validator, NPoS, Election, Stake

### Archive Node

**Definition**: An RPC node that maintains complete historical state for all blocks. Archive nodes can query any block from genesis to present, unlike regular RPC nodes which only maintain recent state (~256 blocks).

**Key Characteristics**:
- Maintains complete historical state
- Can query any block from genesis to present
- Requires significantly more storage (hundreds of GB to TB)
- Slower queries due to larger database size
- Essential for historical block queries

**Related Terms**: RPC, Block Number, Historical Block, Regular Node

## Technical Terms

### RPC (Remote Procedure Call)

**Definition**: A protocol for querying blockchain data from remote nodes. The tool uses RPC endpoints to fetch validator and nominator data from live Substrate chains.

**Key Characteristics**:
- Protocol for remote data queries
- Used to fetch blockchain data
- Supports multiple RPC methods
- Can query latest or historical blocks
- Requires network access

**Related Terms**: Archive Node, Block Number, Substrate

### Substrate

**Definition**: A blockchain framework used by Polkadot and many other chains. Substrate provides the building blocks for creating custom blockchains, including consensus mechanisms, runtime modules, and RPC interfaces.

**Key Characteristics**:
- Blockchain framework
- Used by Polkadot and many chains
- Provides consensus mechanisms
- Includes runtime modules
- Supports custom blockchains

**Related Terms**: Polkadot, RPC, Election Algorithm, Runtime

### SS58

**Definition**: An encoding format used for Polkadot account addresses. SS58 is a base-58 encoding scheme with checksums, similar to Bitcoin addresses but with chain-specific prefixes.

**Key Characteristics**:
- Base-58 encoding with checksums
- Used for account addresses
- Chain-specific prefixes
- Similar to Bitcoin addresses
- Validates account ID formats

**Related Terms**: Account ID, Address, Encoding

### Bit-for-Bit Accuracy

**Definition**: Producing identical results to on-chain elections using the same algorithms and data. The tool achieves bit-for-bit accuracy by using Substrate's native crates and algorithms.

**Key Characteristics**:
- Identical results to on-chain elections
- Uses same Substrate crates
- Uses same election algorithms
- Processes same input data
- Follows same election logic

**Related Terms**: Substrate, Election Algorithm, Accuracy, Validation

### SCALE Codec

**Definition**: A compact binary encoding format used by Substrate for serializing data. SCALE (Simple Concatenated Aggregate Little-Endian) is used for encoding storage values, RPC parameters, and blockchain data.

**Key Characteristics**:
- Compact binary encoding
- Used by Substrate
- Encodes storage values
- Encodes RPC parameters
- Efficient serialization

**Related Terms**: Substrate, RPC, Storage, Encoding

### TwoX128 Hashing

**Definition**: A hashing algorithm used by Substrate for generating storage keys. TwoX128 is used to hash pallet and storage item names to create storage keys.

**Key Characteristics**:
- Hashing algorithm for storage keys
- Used by Substrate
- Hashes pallet names
- Hashes storage item names
- Creates storage keys

**Related Terms**: Storage Key, Substrate, RPC, Storage

### Storage Key

**Definition**: A key used to access storage values in Substrate chains. Storage keys are generated by hashing pallet and storage item names (e.g., `twox128("Staking") + twox128("Validators")`).

**Key Characteristics**:
- Key for accessing storage values
- Generated by hashing pallet names
- Generated by hashing storage item names
- Used in RPC queries
- Chain-specific

**Related Terms**: TwoX128 Hashing, Substrate, RPC, Storage

### Block Hash

**Definition**: A cryptographic hash of a block's contents. Block hashes are used to uniquely identify blocks and query historical state.

**Key Characteristics**:
- Cryptographic hash of block contents
- Uniquely identifies blocks
- Used for historical queries
- Retrieved via RPC
- Required for storage queries

**Related Terms**: Block Number, Archive Node, RPC, Historical Block

### Block Number

**Definition**: A sequential number assigned to each block in a blockchain. Block numbers are used to identify specific blocks and query historical state.

**Key Characteristics**:
- Sequential number for each block
- Identifies specific blocks
- Used for historical queries
- Increases monotonically
- Retrieved via RPC

**Related Terms**: Block Hash, Archive Node, RPC, Historical Block

### Election Provider

**Definition**: A component that provides election functionality in Substrate chains. Election providers implement the `ElectionProvider` trait and can be customized for different chains.

**Key Characteristics**:
- Provides election functionality
- Implements `ElectionProvider` trait
- Can be customized per chain
- Used by runtime modules
- Supports different algorithms

**Related Terms**: Election Algorithm, Substrate, Runtime, NPoS

## Cross-References

### Terms by Category

**Algorithm Terms**: NPoS, Sequential Phragmen, Parallel Phragmen, Multi-phase, Phragmen

**Polkadot Ecosystem Terms**: Validator, Nominator, Stake, Active Set, Archive Node

**Technical Terms**: RPC, Substrate, SS58, Bit-for-Bit Accuracy, SCALE Codec, TwoX128 Hashing, Storage Key, Block Hash, Block Number, Election Provider

### Related Concepts

**Election Process**: NPoS, Validator, Nominator, Active Set, Stake, Sequential Phragmen, Parallel Phragmen, Multi-phase

**Data Access**: RPC, Archive Node, Block Number, Block Hash, Storage Key, SCALE Codec

**Accuracy & Validation**: Bit-for-Bit Accuracy, Substrate, Election Algorithm, Election Provider

## Notes

- All terms are defined where they first appear in documentation
- This glossary provides comprehensive definitions for reference
- Terms are organized by category for easy navigation
- Cross-references help find related terms

