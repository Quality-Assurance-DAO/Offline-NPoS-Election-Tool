# Quickstart: Comprehensive Test Enhancement

**Date**: 2025-01-27  
**Feature**: 002-comprehensive-testing

## Overview

This quickstart guide explains how to use the comprehensive test suite enhancements, including edge case tests, performance benchmarks, chain snapshot regression tests, and regression test fixtures.

## Prerequisites

- Rust 1.70+ installed
- Project dependencies installed (`cargo build`)
- For chain snapshot tests: Access to public Substrate RPC endpoints (optional, snapshots can be pre-fetched)

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Specific Test Categories

```bash
# Edge case tests
cargo test --test edge_cases

# Performance benchmarks
cargo test --test performance

# Chain snapshot regression tests
cargo test --test chain_snapshots

# Regression tests
cargo test --test regression
```

### Run Individual Tests

```bash
# Run specific edge case test
cargo test --test edge_cases test_zero_candidates

# Run specific performance benchmark
cargo test --test performance test_large_scale_1k_candidates -- --nocapture

# Run specific chain snapshot test
cargo test --test chain_snapshots test_polkadot_block_12345678
```

## Edge Case Tests

Edge case tests validate boundary conditions and unusual inputs.

### Example: Test Zero Candidates

```bash
cargo test --test edge_cases test_zero_candidates
```

**Expected Behavior**: Test should fail with clear error message indicating insufficient candidates.

### Available Edge Case Tests

- `test_zero_candidates` - Election with zero validator candidates
- `test_zero_nominators` - Election with zero nominators
- `test_single_candidate` - Election with exactly one candidate
- `test_single_nominator` - Election with exactly one nominator
- `test_max_active_set_size` - Active set size equals candidate count
- `test_malformed_json` - Invalid JSON input handling
- `test_invalid_account_ids` - Invalid SS58 account ID validation
- `test_duplicate_account_ids` - Duplicate account ID detection
- `test_empty_voting_edges` - Nominators with no voting targets
- `test_all_nominators_vote_all` - Extreme voting pattern: all nominators vote for all candidates
- `test_each_nominator_votes_one` - Extreme voting pattern: each nominator votes for one candidate
- `test_zero_stakes` - Candidates/nominators with zero stake
- `test_maximum_stakes` - Stake values near u128 maximum
- `test_circular_dependencies` - Unusual voting graph structures

## Performance Benchmarks

Performance benchmarks measure execution time and memory usage for large-scale elections.

### Run Performance Benchmarks

```bash
# Run all performance benchmarks
cargo test --test performance -- --nocapture

# Run specific benchmark
cargo test --test performance test_large_scale_1k_candidates -- --nocapture
```

### Benchmark Output

Benchmarks output structured JSON to stdout:

```json
{
  "timing_ms": 1234,
  "memory_mb": 256,
  "metadata": {
    "benchmark_name": "large_scale_1k_candidates",
    "candidate_count": 1000,
    "nominator_count": 10000,
    "algorithm": "sequential-phragmen",
    "iterations": 10,
    "mean_time_ms": 1234.5,
    "std_dev_ms": 45.2
  }
}
```

### Available Performance Benchmarks

- `test_large_scale_1k_candidates` - 1,000 candidates, 10,000 nominators (target: < 60s)
- `test_large_scale_5k_candidates` - 5,000 candidates, 50,000 nominators (target: < 5min)
- `test_large_scale_10k_candidates` - 10,000 candidates, 100,000 nominators (target: no OOM)
- `test_memory_leak_detection` - 100 consecutive elections, memory stability check

### Save Benchmark Results

```bash
# Run benchmark and save to file
cargo test --test performance test_large_scale_1k_candidates -- --nocapture > benchmark_results.json
```

## Chain Snapshot Regression Tests

Chain snapshot tests validate accuracy against real on-chain election results.

### Run Chain Snapshot Tests

```bash
# Run all chain snapshot tests
cargo test --test chain_snapshots

# Run tests for specific chain
cargo test --test chain_snapshots -- polkadot

# Run test for specific block
cargo test --test chain_snapshots test_polkadot_block_12345678
```

### RPC Retry Behavior

Chain snapshot tests automatically retry RPC calls with exponential backoff:
- Attempt 1: Immediate
- Attempt 2: After 1 second delay
- Attempt 3: After 2 second delay
- Attempt 4: After 4 second delay

If all retries fail, test is marked as skipped with reason: "RPC endpoint unavailable after 3 retries"

### Fetching New Chain Snapshots

To fetch new chain snapshot data:

```bash
# Fetch snapshot for specific block (example)
cargo run --bin offline-election -- fetch-snapshot \
  --chain polkadot \
  --block-number 12345678 \
  --output tests/fixtures/chain_snapshots/polkadot/12345678.json
```

### Available Chain Snapshots

Chain snapshots are stored in `tests/fixtures/chain_snapshots/`:
- `polkadot/` - Polkadot chain snapshots
- `kusama/` - Kusama chain snapshots
- `[other-chains]/` - Other Substrate chain snapshots

Each snapshot file contains:
- Chain metadata (chain identifier, block number, timestamp)
- Election data (candidates, nominators)
- Expected on-chain results (selected validators, stake allocations)

## Regression Tests

Regression tests ensure election results remain consistent across code changes.

### Run Regression Tests

```bash
# Run all regression tests
cargo test --test regression

# Run specific regression test
cargo test --test regression test_normal_election_5x5
```

### Test Fixture Structure

Regression test fixtures are stored in `tests/fixtures/regression/`:

```json
{
  "metadata": {
    "test_name": "normal_election_5x5",
    "description": "Normal election with 5 candidates and 5 nominators",
    "created": "2025-01-27T10:00:00Z",
    "algorithm": "sequential-phragmen",
    "category": "regression"
  },
  "input": {
    "candidates": [...],
    "nominators": [...],
    "config": {...}
  },
  "expected_result": {
    "selected_validators": [...],
    "stake_allocations": {...}
  }
}
```

### Creating New Regression Tests

1. Create test fixture JSON file in `tests/fixtures/regression/`
2. Include input data and expected results
3. Add test function in `tests/integration/regression/`
4. Test compares actual results to expected results

## Test Fixtures

### Test Fixture Locations

- Edge case fixtures: `tests/fixtures/regression/edge_cases/`
- Regression fixtures: `tests/fixtures/regression/`
- Chain snapshots: `tests/fixtures/chain_snapshots/{chain}/`
- Benchmark results: `tests/fixtures/benchmarks/`

### Loading Test Fixtures

Test fixtures are automatically loaded by test framework:

```rust
#[test]
fn test_example() {
    let fixture = load_test_fixture("normal_election_5x5")?;
    let result = run_election(&fixture.input)?;
    assert_eq!(result, fixture.expected_result);
}
```

## Continuous Integration

### Running Tests in CI

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test --all-features -- --nocapture

# Run benchmarks (may take longer)
cargo test --test performance -- --nocapture
```

### Performance Regression Detection

Compare benchmark results across CI runs:
- Compare `mean_time_ms` values
- Detect if execution time increases > 10%
- Compare `memory_mb` values
- Fail CI if performance regresses beyond threshold

## Troubleshooting

### RPC Failures

If chain snapshot tests fail due to RPC issues:
- Check network connectivity
- Verify RPC endpoint is accessible
- Tests will retry automatically (up to 3 attempts)
- If all retries fail, test is skipped (not failed)

### Performance Test Failures

If performance benchmarks fail:
- Check system resources (CPU, memory)
- Verify hardware meets minimum requirements (4 CPU cores, 8GB RAM)
- Review benchmark output for timing/memory details
- Compare against previous benchmark results

### Test Fixture Errors

If test fixtures fail to load:
- Verify JSON syntax is valid
- Check required fields are present
- Validate account IDs are valid SS58 format
- Ensure voting targets reference existing candidates

## Next Steps

- Review test results and address any failures
- Add new test fixtures for additional scenarios
- Update expected results when intentional algorithm changes occur
- Fetch new chain snapshots periodically to maintain coverage

