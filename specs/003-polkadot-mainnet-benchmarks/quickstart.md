# Quickstart: Polkadot Mainnet Performance Benchmarks

**Date**: 2025-01-27  
**Feature**: 003-polkadot-mainnet-benchmarks  
**Status**: Design Complete

## Overview

This quickstart guide explains how to run performance benchmarks with real Polkadot mainnet data. Benchmarks measure execution time and memory usage for elections using actual on-chain data from recent Polkadot mainnet blocks.

## Prerequisites

- Rust 1.70 or later
- Network access to Polkadot mainnet RPC endpoints
- Cargo and standard Rust toolchain installed

## Quick Start

### 1. Run a Single Benchmark Test

Run a performance benchmark with default settings (recent block within last 30 days):

```bash
cargo test --test test_polkadot_mainnet_performance -- --ignored --nocapture
```

This will:
- Fetch real Polkadot mainnet data from a recent block
- Run election simulation with sequential-phragmen algorithm
- Measure execution time and memory usage
- Validate against performance thresholds
- Output structured JSON results

### 2. Run Benchmark with Specific Block Number

To benchmark with a specific block number:

```rust
// In test file, set block number:
const BLOCK_NUMBER: u64 = 20_000_000; // Your desired block number
```

Or modify the test to accept block number as parameter.

### 3. Run Benchmarks for All Algorithms

Run benchmarks for all three election algorithms:

```bash
# Sequential Phragmen
cargo test --test test_polkadot_mainnet_performance_sequential -- --ignored --nocapture

# Parallel Phragmen
cargo test --test test_polkadot_mainnet_performance_parallel -- --ignored --nocapture

# Multi-Phase
cargo test --test test_polkadot_mainnet_performance_multiphase -- --ignored --nocapture
```

### 4. View Benchmark Results

Benchmark results are stored in two locations:

**JSON Files**: `tests/fixtures/benchmarks/polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json`

**Markdown Documentation**: `PERFORMANCE_BENCHMARKS.md` (repository root)

Example JSON output:

```json
{
  "timing_ms": 15234,
  "memory_mb": 342,
  "metadata": {
    "benchmark_name": "polkadot_mainnet",
    "candidate_count": 387,
    "nominator_count": 23456,
    "algorithm": "sequential-phragmen",
    "block_number": 20000000,
    "chain": "polkadot",
    "threshold_ms": 30000,
    "threshold_passed": true,
    "timestamp": "2025-01-27T12:00:00Z"
  }
}
```

## Configuration

### RPC Endpoint Selection

Default RPC endpoint: `https://polkadot.api.onfinality.io/public`

Alternative endpoints:
- `https://rpc.polkadot.io`
- `https://polkadot-rpc.dwellir.com`

To use a different endpoint, modify the `RPC_URL` constant in the test file:

```rust
const RPC_URL: &str = "https://rpc.polkadot.io";
```

### Block Number Selection

**Default**: Recent block within last 30 days (calculated automatically)

**Override**: Specify a specific block number:

```rust
const BLOCK_NUMBER: u64 = 20_000_000;
```

**Note**: For historical blocks (>30 days old), use archive node endpoints:
- `https://polkadot.api.onfinality.io/public` (archive node)
- `https://polkadot-rpc.dwellir.com` (archive node)

### Performance Thresholds

Algorithm-specific thresholds are automatically validated:

| Algorithm | Threshold |
|-----------|-----------|
| Sequential-phragmen | < 30s |
| Parallel-phragmen | < 15s |
| Multi-phase | < 45s |

Benchmark test fails if execution time exceeds threshold.

## Example Test Code

### Basic Benchmark Test

```rust
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
    const BLOCK_NUMBER: u64 = 20_000_000; // Recent block
    
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
    
    // Validate threshold
    const THRESHOLD_MS: u64 = 30_000; // 30 seconds for sequential-phragmen
    assert!(
        execution_time_ms <= THRESHOLD_MS,
        "Execution time {}ms exceeds threshold {}ms",
        execution_time_ms,
        THRESHOLD_MS
    );
    
    // Output benchmark results
    let mut metadata = HashMap::new();
    metadata.insert("benchmark_name".to_string(), "polkadot_mainnet".to_string());
    metadata.insert("candidate_count".to_string(), election_data.candidates.len().to_string());
    metadata.insert("nominator_count".to_string(), election_data.nominators.len().to_string());
    metadata.insert("algorithm".to_string(), "sequential-phragmen".to_string());
    metadata.insert("block_number".to_string(), BLOCK_NUMBER.to_string());
    metadata.insert("chain".to_string(), "polkadot".to_string());
    
    let benchmark_results = create_benchmark_results(
        execution_time_ms,
        0, // Memory measurement - will be enhanced
        0,
        1,
        metadata,
    );
    
    let json_output = output_benchmark_json(&benchmark_results).unwrap();
    println!("Benchmark results:\n{}", json_output);
    
    println!("✓ Polkadot mainnet benchmark completed: {}ms", execution_time_ms);
}
```

### Benchmark with Memory Measurement

```rust
use crate::common::benchmark_utils::{measure_execution_time, measure_memory_usage, create_benchmark_results};

// Measure memory before and after execution
let memory_before = measure_memory_usage();
let (result, duration) = measure_execution_time(|| {
    engine.execute(&config, &election_data)
});
let memory_after = measure_memory_usage();

let memory_peak_mb = if memory_after > memory_before {
    memory_after - memory_before
} else {
    0 // Measurement unavailable
};

let benchmark_results = create_benchmark_results(
    execution_time_ms,
    memory_peak_mb,
    memory_after,
    1,
    metadata,
);
```

## Troubleshooting

### RPC Endpoint Failures

If RPC endpoint fails:

1. **Check network connectivity**: Ensure you have internet access
2. **Try alternative endpoints**: Use different RPC endpoint URLs
3. **Check endpoint status**: Verify RPC endpoint is operational
4. **Review error messages**: Error messages include suggested alternative endpoints

Example error handling:

```rust
let election_data = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(loader.load_at_block(BLOCK_NUMBER))
    .map_err(|e| {
        eprintln!("Failed to fetch data: {}", e);
        eprintln!("Try alternative endpoints:");
        eprintln!("- https://rpc.polkadot.io");
        eprintln!("- https://polkadot-rpc.dwellir.com");
        e
    })?;
```

### Memory Measurement Unavailable

If memory measurement returns 0:

- **Linux/macOS/Windows**: Memory measurement should work automatically
- **Other platforms**: Graceful degradation - benchmark completes with `memory_mb: 0`
- **Check platform support**: Verify platform is supported in `measure_memory_usage()` implementation

### Threshold Violations

If benchmark exceeds threshold:

- **Review hardware**: Ensure adequate CPU and RAM resources
- **Check system load**: Reduce system load during benchmark execution
- **Verify block number**: Ensure block number is recent and valid
- **Review algorithm**: Different algorithms have different thresholds

## Best Practices

1. **Run benchmarks on dedicated hardware**: Minimize system load for consistent results
2. **Use archive node endpoints**: For historical blocks, use archive node endpoints
3. **Document hardware specifications**: Include CPU, RAM, and OS details in results
4. **Run multiple iterations**: For statistical analysis, run multiple iterations
5. **Track trends over time**: Compare results across different runs to detect regressions
6. **Use consistent block numbers**: For regression testing, use the same block number

## Next Steps

- Review [data-model.md](./data-model.md) for detailed data model documentation
- Review [contracts/polkadot-mainnet-benchmark-output.md](./contracts/polkadot-mainnet-benchmark-output.md) for output format specification
- Review [research.md](./research.md) for implementation details
- Check `PERFORMANCE_BENCHMARKS.md` for current benchmark results

## Related Documentation

- [PERFORMANCE_BENCHMARKS.md](../../../PERFORMANCE_BENCHMARKS.md) - Current benchmark results and performance characteristics
- [RPC_ARCHIVE_NODES.md](../../../RPC_ARCHIVE_NODES.md) - RPC endpoint information
- [tests/README.md](../../../tests/README.md) - Test infrastructure documentation

