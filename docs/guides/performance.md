# Performance Benchmarks and Large-Scale Testing

This document describes the performance benchmarking capabilities of the tool, current benchmark results (where available), and how to benchmark with real-world Polkadot mainnet data.

## Overview

The tool includes performance tests and benchmarks, but **most benchmarks use synthetic data**. This document clarifies:
- What benchmarks exist
- What they measure
- How to benchmark with real Polkadot mainnet data
- Current performance characteristics

## Current Benchmark Status

### ✅ Synthetic Benchmarks (Implemented)

The tool includes comprehensive synthetic benchmarks:

#### Integration Tests (`tests/integration/performance/`)

These tests use **synthetic data** generated programmatically:

| Test | Candidates | Nominators | Target | Status |
|------|-----------|------------|--------|--------|
| `test_large_scale_1k` | 1,000 | 10,000 | < 60s | ✅ Implemented |
| `test_large_scale_5k` | 5,000 | 50,000 | < 5min | ✅ Implemented |
| `test_large_scale_10k` | 10,000 | 100,000 | No OOM | ✅ Implemented |
| `test_large_nominee_sets` | 500 | 10,000+ | - | ✅ Implemented |
| `test_dense_voting` | Variable | Variable | - | ✅ Implemented |
| `test_sparse_voting` | Variable | Variable | - | ✅ Implemented |
| `test_memory_leak` | Variable | Variable | Memory stable | ✅ Implemented |

**Limitations**:
- Use synthetic data patterns (may not reflect real-world voting patterns)
- Don't test with actual Polkadot mainnet nominator/candidate counts
- Don't account for real-world stake distributions

#### Criterion Benchmarks (`benches/large_scale_benchmark.rs`)

Criterion-based benchmarks for statistical analysis:

```bash
cargo bench
```

Benchmarks scales:
- 100 candidates, 1,000 nominators
- 500 candidates, 5,000 nominators
- 1,000 candidates, 10,000 nominators
- 2,000 candidates, 20,000 nominators
- 5,000 candidates, 50,000 nominators

**Limitations**:
- Also use synthetic data
- Don't include Polkadot mainnet scale (typically ~300-400 validators, ~20k+ nominators)

### ⚠️ Real-World Benchmarks (Limited)

#### Chain Snapshot Tests (`tests/integration/chain_snapshots/`)

These tests use **real on-chain data** but focus on **accuracy**, not performance:

- `test_polkadot.rs` - 4 historical Polkadot blocks
- `test_kusama.rs` - 4 historical Kusama blocks
- `test_westend.rs` - 2 historical Westend blocks

**What they measure**:
- ✅ Accuracy (results match on-chain outcomes)
- ❌ Performance (execution time not benchmarked)
- ❌ Memory usage (not measured)

**Scale**:
- Polkadot: ~300-400 validators, ~20k-30k nominators (varies by era)
- Kusama: Similar scale
- Westend: Smaller scale (testnet)

## Polkadot Mainnet Scale

### Typical Polkadot Mainnet Characteristics

As of 2024, Polkadot mainnet typically has:

- **Validators**: ~300-400 candidates
- **Active Set**: 297 validators (fixed)
- **Nominators**: ~20,000-30,000 nominators
- **Voting Patterns**: 
  - Most nominators vote for 1-16 validators (typical)
  - Some vote for more (up to 16 is the limit)
  - Voting patterns are sparse (not all nominators vote for all validators)

### Current Benchmark Coverage

| Scale | Synthetic Tests | Real-World Tests | Status |
|-------|----------------|------------------|--------|
| **Polkadot Mainnet Scale** (~300-400 candidates, ~20k-30k nominators) | ❌ Not covered | ⚠️ Accuracy only, no performance | **Gap** |
| **Small Scale** (100-1k candidates, 1k-10k nominators) | ✅ Covered | ⚠️ Limited | Partial |
| **Large Scale** (5k-10k candidates, 50k-100k nominators) | ✅ Covered | ❌ Not covered | Synthetic only |

## How to Benchmark with Real Polkadot Mainnet Data

### Method 1: Benchmark Using Chain Snapshots

1. **Fetch real Polkadot mainnet data**:

```bash
# Fetch data from a recent block
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 297 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number <RECENT_BLOCK_NUMBER> \
  --output-file polkadot_mainnet_snapshot.json
```

2. **Run performance benchmark**:

```bash
# Use the fetched data for benchmarking
time offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 297 \
  --input-file polkadot_mainnet_snapshot.json \
  --output-file /dev/null
```

3. **Measure memory usage** (requires additional tools):

```bash
# On Linux, use /usr/bin/time
/usr/bin/time -v offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 297 \
  --input-file polkadot_mainnet_snapshot.json \
  --output-file /dev/null
```

### Method 2: Create a Benchmark Script

Create a script to automate benchmarking:

```bash
#!/bin/bash
# benchmark_polkadot_mainnet.sh

BLOCK_NUMBER=$1
RPC_URL="https://polkadot.api.onfinality.io/public"

echo "Fetching Polkadot mainnet data from block $BLOCK_NUMBER..."

# Fetch data
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 297 \
  --rpc-url "$RPC_URL" \
  --block-number "$BLOCK_NUMBER" \
  --output-file /tmp/polkadot_snapshot.json

echo "Running performance benchmark..."

# Benchmark
time offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 297 \
  --input-file /tmp/polkadot_snapshot.json \
  --output-file /dev/null

# Cleanup
rm /tmp/polkadot_snapshot.json
```

Usage:
```bash
chmod +x benchmark_polkadot_mainnet.sh
./benchmark_polkadot_mainnet.sh <BLOCK_NUMBER>
```

### Method 3: Add Performance Benchmark Test

Create a new test file `tests/integration/performance/test_polkadot_mainnet.rs`:

```rust
//! Performance benchmark using real Polkadot mainnet data
//! This test fetches real data and benchmarks execution time

use offline_election::engine::ElectionEngine;
use offline_election::models::election_config::ElectionConfiguration;
use offline_election::types::AlgorithmType;
use offline_election::input::rpc::RpcLoader;
use crate::common::benchmark_utils::{measure_execution_time, create_benchmark_results, output_benchmark_json};
use std::collections::HashMap;

#[test]
#[ignore] // Requires network access
fn test_polkadot_mainnet_performance() {
    const RPC_URL: &str = "https://polkadot.api.onfinality.io/public";
    const BLOCK_NUMBER: u64 = 20_000_000; // Adjust to recent block
    
    println!("Fetching Polkadot mainnet data from block {}...", BLOCK_NUMBER);
    
    // Fetch real data
    let loader = RpcLoader::new(RPC_URL).unwrap();
    let election_data = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(loader.load_at_block(BLOCK_NUMBER))
        .unwrap();
    
    println!("Fetched: {} candidates, {} nominators", 
             election_data.candidates.len(), 
             election_data.nominators.len());
    
    let engine = ElectionEngine::new();
    let config = ElectionConfiguration {
        active_set_size: 297, // Polkadot active set size
        algorithm: AlgorithmType::SequentialPhragmen,
        overrides: None,
        block_number: Some(BLOCK_NUMBER),
    };
    
    println!("Running performance benchmark...");
    let (result, duration) = measure_execution_time(|| {
        engine.execute(&config, &election_data)
    });
    
    let execution_time_ms = duration.as_millis() as u64;
    
    assert!(result.is_ok(), "Election should succeed");
    let election_result = result.unwrap();
    
    assert_eq!(election_result.selected_validators.len(), 297);
    
    // Output benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "polkadot_mainnet".to_string());
    metadata.insert("candidate_count".to_string(), election_data.candidates.len().to_string());
    metadata.insert("nominator_count".to_string(), election_data.nominators.len().to_string());
    metadata.insert("algorithm".to_string(), "sequential-phragmen".to_string());
    metadata.insert("block_number".to_string(), BLOCK_NUMBER.to_string());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        0, // Memory measurement not implemented
        0,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Polkadot mainnet benchmark completed: {}ms", execution_time_ms);
}
```

## Running Existing Benchmarks

### Synthetic Benchmarks

```bash
# Run all performance tests
cargo test --test '*performance*' -- --ignored --nocapture

# Run specific test
cargo test test_large_scale_1k_candidates_10k_nominators -- --ignored --nocapture

# Run Criterion benchmarks
cargo bench
```

### Chain Snapshot Tests (Accuracy, Not Performance)

```bash
# Run chain snapshot tests (validates accuracy, not performance)
cargo test --test '*chain_snapshots*' -- --ignored
```

## Performance Characteristics

### Expected Performance (Based on Synthetic Benchmarks)

| Scale | Candidates | Nominators | Expected Time | Algorithm |
|-------|-----------|------------|---------------|------------|
| Small | 100-500 | 1k-5k | < 1s | Sequential Phragmen |
| Medium | 500-1k | 5k-10k | < 10s | Sequential Phragmen |
| Large | 1k-5k | 10k-50k | < 5min | Sequential Phragmen |
| Very Large | 5k-10k | 50k-100k | < 30min | Sequential Phragmen |

**Note**: These are estimates based on synthetic data. Real-world performance may vary based on:
- Voting pattern density
- Stake distribution
- Hardware specifications

### Polkadot Mainnet Scale Estimates

Based on synthetic benchmarks and algorithm complexity:

| Metric | Estimate | Notes |
|--------|----------|-------|
| **Execution Time** | 5-30 seconds | Depends on voting patterns and hardware |
| **Memory Usage** | 100-500 MB | Depends on data structures |
| **Scalability** | Linear with nominators | O(n) complexity for Sequential Phragmen |

**⚠️ These are estimates. Actual benchmarks needed.**

## Known Performance Gaps

### 1. No Real-World Performance Benchmarks

**Gap**: No documented performance benchmarks using actual Polkadot mainnet data.

**Impact**: 
- Unclear if tool performs well at production scale
- No baseline for performance regression testing
- Users can't predict execution time for real-world scenarios

**Recommendation**: 
- Add performance benchmarks using real Polkadot mainnet snapshots
- Document actual execution times and memory usage
- Include in CI/CD for regression detection

### 2. Limited Memory Measurement

**Gap**: Memory measurement is not implemented (`measure_memory_usage()` returns 0).

**Impact**:
- Can't detect memory leaks or excessive memory usage
- No way to predict memory requirements for large datasets

**Recommendation**:
- Implement proper memory measurement (e.g., using `jemalloc` statistics or platform-specific APIs)
- Add memory benchmarks to CI/CD

### 3. No Performance Regression Testing

**Gap**: Performance tests exist but aren't integrated into regression testing.

**Impact**:
- Performance regressions may go undetected
- No automated alerts for performance degradation

**Recommendation**:
- Add performance regression tests to CI/CD
- Set performance thresholds and fail builds if exceeded
- Track performance trends over time

## Recommendations

### Short Term

1. **Add Polkadot Mainnet Performance Benchmark**
   - Create test using real Polkadot mainnet data
   - Document execution time and memory usage
   - Add to test suite

2. **Implement Memory Measurement**
   - Use platform-specific APIs or `jemalloc` statistics
   - Add memory benchmarks to all performance tests

3. **Document Current Performance**
   - Run benchmarks on multiple scales
   - Document results in this file
   - Include hardware specifications

### Long Term

1. **Continuous Performance Monitoring**
   - Add performance benchmarks to CI/CD
   - Track performance trends
   - Alert on regressions

2. **Real-World Benchmark Suite**
   - Benchmark with multiple real chain snapshots
   - Include different eras (different voting patterns)
   - Compare across algorithms

3. **Performance Optimization**
   - Profile with real-world data
   - Identify bottlenecks
   - Optimize hot paths

## Contributing Benchmarks

If you run benchmarks with real Polkadot mainnet data, please:

1. Document your results in this file
2. Include:
   - Block number used
   - Number of candidates/nominators
   - Execution time
   - Memory usage
   - Hardware specifications
   - Algorithm used

3. Submit a PR with your benchmark results

## Example Benchmark Results

### Synthetic Data (Example)

```json
{
  "timing_ms": 1234,
  "memory_mb": 256,
  "metadata": {
    "benchmark_name": "large_scale_1k",
    "candidate_count": 1000,
    "nominator_count": 10000,
    "algorithm": "sequential-phragmen",
    "iterations": 1
  }
}
```

### Real-World Data (To Be Added)

```json
{
  "timing_ms": 15000,
  "memory_mb": 350,
  "metadata": {
    "benchmark_name": "polkadot_mainnet",
    "candidate_count": 387,
    "nominator_count": 23456,
    "algorithm": "sequential-phragmen",
    "block_number": 20000000,
    "chain": "polkadot"
  }
}
```

## Summary

- ✅ **Synthetic benchmarks exist** for various scales
- ⚠️ **Real-world benchmarks are limited** (accuracy tests only, no performance)
- ❌ **Polkadot mainnet performance not benchmarked** with actual data
- ❌ **Memory measurement not implemented**
- ⚠️ **Performance regression testing not integrated**

**Action Items**:
1. Add performance benchmarks using real Polkadot mainnet data
2. Implement memory measurement
3. Document actual performance characteristics
4. Integrate performance testing into CI/CD

