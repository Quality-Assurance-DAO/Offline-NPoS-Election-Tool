# Election Algorithm Extensibility Guide

This document explains how to add new election algorithms to the Offline NPoS Election Tool.

## Overview

The tool is designed with extensibility in mind. The architecture uses a trait-based system that makes it straightforward to add new election algorithms without modifying core infrastructure.

## Current Algorithms

The tool currently supports three algorithms:

1. **Sequential Phragmen** - Standard algorithm using `sp_npos_elections::seq_phragmen`
2. **Parallel Phragmen** - Alternative algorithm using `sp_npos_elections::phragmms`
3. **Multi-phase** - Multi-phase election wrapper using sequential phragmen internally

## Architecture

### Core Components

1. **`ElectionAlgorithm` Trait** (`src/algorithms/trait_def.rs`)
   - Defines the interface all algorithms must implement
   - Requires `execute()` and `name()` methods

2. **`AlgorithmType` Enum** (`src/types.rs`)
   - Enumeration of all supported algorithm types
   - Used for configuration and serialization

3. **`ElectionEngine`** (`src/engine.rs`)
   - Dispatches to the appropriate algorithm based on configuration
   - Handles validation and result processing

## Adding a New Algorithm

### Step 1: Create Algorithm Module

Create a new file `src/algorithms/your_algorithm.rs`:

```rust
//! Your custom algorithm implementation

use crate::algorithms::trait_def::ElectionAlgorithm;
use crate::error::ElectionError;
use crate::models::election_config::ElectionConfiguration;
use crate::models::election_data::ElectionData;
use crate::models::election_result::{ElectionResult, SelectedValidator, StakeAllocation, ExecutionMetadata};
use std::collections::HashMap;

/// Your custom algorithm implementation
pub struct YourAlgorithm;

impl ElectionAlgorithm for YourAlgorithm {
    fn execute(
        &self,
        data: &ElectionData,
        config: &ElectionConfiguration,
    ) -> Result<ElectionResult, ElectionError> {
        // Validate input
        if data.candidates.is_empty() {
            return Err(ElectionError::ValidationError {
                message: "Cannot run election with zero candidates".to_string(),
                field: None,
            });
        }

        // Convert data to your algorithm's format
        let candidate_lookup: HashMap<String, &crate::models::validator::ValidatorCandidate> = data
            .candidates
            .iter()
            .map(|candidate| (candidate.account_id.clone(), candidate))
            .collect();

        // Build voters list
        let mut voters: Vec<(String, u64, Vec<String>)> = Vec::new();
        for nominator in data.nominators.iter() {
            let targets: Vec<String> = nominator
                .targets
                .iter()
                .filter(|id| candidate_lookup.contains_key(*id))
                .cloned()
                .collect();

            if targets.is_empty() {
                continue;
            }

            let stake_u64 = nominator.stake.min(u64::MAX as u128) as u64;
            voters.push((nominator.account_id.clone(), stake_u64, targets));
        }

        // Execute your algorithm logic here
        // This is where you'd call your algorithm's computation function
        // For example, if using a Substrate crate:
        // let solution = your_crate::your_algorithm(...)?;

        // Convert results to ElectionResult format
        let selected_validators = vec![/* ... */];
        let stake_distribution = vec![/* ... */];
        let total_stake: u128 = data.nominators.iter().map(|n| n.stake).sum();

        Ok(ElectionResult {
            selected_validators,
            stake_distribution,
            total_stake,
            algorithm_used: crate::types::AlgorithmType::YourAlgorithm,
            execution_metadata: ExecutionMetadata {
                block_number: config.block_number,
                execution_timestamp: Some(chrono::Utc::now().to_rfc3339()),
                data_source: None,
            },
            diagnostics: None,
        })
    }

    fn name(&self) -> &'static str {
        "your-algorithm"
    }
}
```

### Step 2: Add to Algorithm Module

Add your algorithm to `src/algorithms/mod.rs`:

```rust
pub mod your_algorithm;

pub use your_algorithm::YourAlgorithm;
```

### Step 3: Add to AlgorithmType Enum

Update `src/types.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlgorithmType {
    SequentialPhragmen,
    ParallelPhragmen,
    MultiPhase,
    YourAlgorithm, // Add your variant
}

impl std::str::FromStr for AlgorithmType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sequential-phragmen" | "sequential" => Ok(AlgorithmType::SequentialPhragmen),
            "parallel-phragmen" | "parallel" => Ok(AlgorithmType::ParallelPhragmen),
            "multi-phase" | "multiphase" => Ok(AlgorithmType::MultiPhase),
            "your-algorithm" | "your" => Ok(AlgorithmType::YourAlgorithm), // Add parsing
            _ => Err(format!("Unknown algorithm type: {}", s)),
        }
    }
}

impl std::fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlgorithmType::SequentialPhragmen => write!(f, "sequential-phragmen"),
            AlgorithmType::ParallelPhragmen => write!(f, "parallel-phragmen"),
            AlgorithmType::MultiPhase => write!(f, "multi-phase"),
            AlgorithmType::YourAlgorithm => write!(f, "your-algorithm"), // Add display
        }
    }
}
```

### Step 4: Register in ElectionEngine

Update `src/engine.rs` to dispatch to your algorithm:

```rust
use crate::algorithms::your_algorithm::YourAlgorithm;

// In execute_with_diagnostics method:
let algorithm: Box<dyn ElectionAlgorithm> = match config.algorithm {
    AlgorithmType::SequentialPhragmen => Box::new(SequentialPhragmen),
    AlgorithmType::ParallelPhragmen => Box::new(ParallelPhragmen),
    AlgorithmType::MultiPhase => Box::new(MultiPhase),
    AlgorithmType::YourAlgorithm => Box::new(YourAlgorithm), // Add case
};
```

### Step 5: Update CLI Help Text (Optional)

If you want the CLI to show your algorithm in help text, update `src/cli/commands.rs`:

```rust
// In the algorithm argument definition:
.arg(
    Arg::new("algorithm")
        .help("Election algorithm to use")
        .value_parser(["sequential-phragmen", "parallel-phragmen", "multi-phase", "your-algorithm"])
        // ...
)
```

## Example: Using Substrate Crates

If your algorithm uses a Substrate crate (like `sp-npos-elections`), you can follow the pattern used by existing algorithms:

```rust
use sp_npos_elections::your_algorithm_function;
use sp_runtime::Perbill;

// Convert candidates
let candidates: Vec<String> = data
    .candidates
    .iter()
    .map(|candidate| candidate.account_id.clone())
    .collect();

// Convert voters
let mut voters: Vec<(String, u64, Vec<String>)> = Vec::new();
for nominator in data.nominators.iter() {
    let targets: Vec<String> = nominator
        .targets
        .iter()
        .filter(|id| candidate_lookup.contains_key(*id))
        .cloned()
        .collect();
    
    if targets.is_empty() {
        continue;
    }
    
    let stake_u64 = nominator.stake.min(u64::MAX as u128) as u64;
    voters.push((nominator.account_id.clone(), stake_u64, targets));
}

// Call Substrate algorithm
let solution = your_algorithm_function::<String, Perbill>(
    config.active_set_size as usize,
    candidates,
    voters,
    None, // or additional parameters
)
.map_err(|e| ElectionError::AlgorithmError {
    message: format!("Your algorithm failed: {:?}", e),
    algorithm: crate::types::AlgorithmType::YourAlgorithm,
})?;

// Convert solution to ElectionResult format
// (see existing algorithms for conversion patterns)
```

## Example: Custom Algorithm Logic

If implementing a completely custom algorithm:

```rust
fn execute(
    &self,
    data: &ElectionData,
    config: &ElectionConfiguration,
) -> Result<ElectionResult, ElectionError> {
    // Your custom election logic here
    // For example, a simple stake-weighted selection:
    
    let mut candidates_with_stake: Vec<_> = data
        .candidates
        .iter()
        .map(|c| (c.account_id.clone(), c.stake))
        .collect();
    
    // Sort by stake descending
    candidates_with_stake.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Select top N
    let selected: Vec<_> = candidates_with_stake
        .iter()
        .take(config.active_set_size as usize)
        .map(|(id, stake)| SelectedValidator {
            account_id: id.clone(),
            total_backing_stake: *stake,
            nominator_count: 0, // Calculate from data if needed
            rank: None,
        })
        .collect();
    
    // Build stake distribution
    let stake_distribution = vec![]; // Calculate from your algorithm
    
    Ok(ElectionResult {
        selected_validators: selected,
        stake_distribution,
        total_stake: data.nominators.iter().map(|n| n.stake).sum(),
        algorithm_used: crate::types::AlgorithmType::YourAlgorithm,
        execution_metadata: ExecutionMetadata {
            block_number: config.block_number,
            execution_timestamp: Some(chrono::Utc::now().to_rfc3339()),
            data_source: None,
        },
        diagnostics: None,
    })
}
```

## Testing Your Algorithm

Add tests to verify your algorithm works correctly:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::election_data::ElectionData;
    use crate::models::election_config::ElectionConfiguration;
    use crate::types::AlgorithmType;

    #[test]
    fn test_your_algorithm() {
        let mut data = ElectionData::new();
        // Add test candidates and nominators
        
        let config = ElectionConfiguration {
            algorithm: AlgorithmType::YourAlgorithm,
            active_set_size: 2,
            overrides: None,
            block_number: None,
        };
        
        let algorithm = YourAlgorithm;
        let result = algorithm.execute(&data, &config);
        
        assert!(result.is_ok());
        // Add more assertions
    }
}
```

## Integration with API

Once added, your algorithm will automatically be available via:

- **CLI**: `offline-election run --algorithm your-algorithm ...`
- **REST API**: `POST /elections/run` with `"algorithm": "your-algorithm"`
- **Programmatic API**: `ElectionConfiguration::new().algorithm(AlgorithmType::YourAlgorithm)`

## Substrate Election Algorithms Reference

### Standard Algorithms in `sp-npos-elections`

The `sp-npos-elections` crate provides several election algorithms:

- **`seq_phragmen`**: Sequential Phragmen ✅ (currently implemented)
- **`phragmms`**: Parallel Phragmen / Phragmms ✅ (currently implemented)
- **`balance`**: Balance-based selection ❌ (not currently implemented)
- **`minimize_supports`**: Minimize supports algorithm ❌ (not currently implemented)

**Note on Missing Algorithms:**
- `balance` and `minimize_supports` are less commonly used in production Substrate chains
- Most chains use sequential phragmen or parallel phragmen
- Multi-phase elections (used by Polkadot/Kusama) use sequential phragmen internally
- If you need these algorithms, they can be added following the extensibility guide above

### Checking Available Algorithms

To see what algorithms are available in your version of `sp-npos-elections`:

```bash
# Check Cargo.toml for the version
grep sp-npos-elections Cargo.toml

# Then check the crate documentation:
# https://docs.rs/sp-npos-elections/
```

Or inspect the crate source:
```bash
cargo doc --open --package sp-npos-elections
```

### Multi-phase Elections

- **`pallet-election-provider-multi-phase`**: Uses sequential phragmen internally (currently implemented)

### Custom Election Providers

Chains can implement custom election providers via the `ElectionProvider` trait. These are chain-specific and require:

1. Understanding the chain's election provider implementation
2. Extracting the algorithm logic
3. Implementing it following the pattern above

## Best Practices

1. **Use Substrate Crates When Possible**: If your algorithm exists in Substrate crates, use them for accuracy
2. **Match On-Chain Behavior**: Ensure your implementation matches on-chain behavior exactly
3. **Handle Edge Cases**: Test with zero candidates, zero nominators, empty targets, etc.
4. **Document Assumptions**: Document any assumptions or differences from standard algorithms
5. **Add Tests**: Include comprehensive tests for your algorithm
6. **Update Documentation**: Update this guide and README with your new algorithm

## Troubleshooting

**Algorithm not recognized:**
- Check `AlgorithmType` enum includes your variant
- Verify `FromStr` implementation includes parsing for your algorithm name
- Ensure CLI/API use the correct string name

**Results don't match on-chain:**
- Verify you're using the same algorithm as the chain
- Check data conversion (account IDs, stakes, targets)
- Ensure active set size matches
- Compare with existing algorithm implementations

**Compilation errors:**
- Ensure all imports are correct
- Check trait implementation matches exactly
- Verify `ElectionResult` construction includes all required fields

## Contributing

When contributing a new algorithm:

1. Follow the existing code style
2. Add comprehensive tests
3. Update documentation
4. Ensure it works with all data sources (RPC, JSON, synthetic)
5. Test with edge cases
6. Add examples to API_USAGE.md

## Questions?

For questions about adding algorithms or understanding the architecture:
- Review existing algorithm implementations in `src/algorithms/`
- Check the trait definition in `src/algorithms/trait_def.rs`
- See how algorithms are dispatched in `src/engine.rs`

