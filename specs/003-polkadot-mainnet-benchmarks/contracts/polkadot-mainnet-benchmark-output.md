# Polkadot Mainnet Benchmark Output Contract

**Date**: 2025-01-27  
**Feature**: 003-polkadot-mainnet-benchmarks  
**Status**: Design Complete

## Overview

This document defines the contract for Polkadot mainnet performance benchmark output. This contract extends the general benchmark output contract (from feature 002) with Polkadot-specific fields and validation rules.

## Output Format

Polkadot mainnet benchmarks output structured JSON with timing, memory usage, and Polkadot-specific metadata. Results are stored in `tests/fixtures/benchmarks/` directory and appended to `PERFORMANCE_BENCHMARKS.md` markdown documentation.

## Schema

### Polkadot Mainnet Benchmark Output Schema

Extends the base benchmark output schema with Polkadot-specific fields:

```json
{
  "timing_ms": "u64 (required, execution time in milliseconds)",
  "memory_mb": "u64 (required, peak memory usage in MB, 0 if unavailable)",
  "metadata": {
    "benchmark_name": "string (required, e.g., 'polkadot_mainnet')",
    "candidate_count": "u32 (required, typically ~300-400 for Polkadot mainnet)",
    "nominator_count": "u32 (required, typically ~20k-30k for Polkadot mainnet)",
    "algorithm": "sequential-phragmen | parallel-phragmen | multi-phase (required)",
    "block_number": "u64 (required, Polkadot mainnet block number)",
    "chain": "string (required, 'polkadot')",
    "rpc_endpoint": "string (optional, RPC endpoint URL used)",
    "iterations": "u32 (optional, number of iterations run, default: 1)",
    "mean_time_ms": "f64 (optional, mean execution time if iterations > 1)",
    "std_dev_ms": "f64 (optional, standard deviation if iterations > 1)",
    "threshold_ms": "u64 (optional, algorithm-specific performance threshold)",
    "threshold_passed": "boolean (optional, true if execution_time_ms <= threshold_ms)",
    "hardware_info": {
      "cpu_cores": "u32 (optional)",
      "ram_gb": "u32 (optional)",
      "os": "string (optional, e.g., 'Linux', 'macOS', 'Windows')",
      "cpu_model": "string (optional)"
    } (optional),
    "environment_info": {
      "rust_version": "string (optional, e.g., '1.70.0')",
      "cargo_version": "string (optional)",
      "test_runner": "string (optional, e.g., 'cargo test')"
    } (optional),
    "timestamp": "ISO 8601 datetime (required, e.g., '2025-01-27T12:00:00Z')",
    "memory_measurement_available": "boolean (optional, false if memory measurement failed)"
  }
}
```

## Validation Rules

### Timing Validation

1. **Execution Time**:
   - `timing_ms` must be non-negative
   - If `iterations > 1`, `mean_time_ms` and `std_dev_ms` must be present
   - `min_time_ms` <= `mean_time_ms` <= `max_time_ms` (if all present)

2. **Threshold Validation**:
   - `threshold_ms` must match algorithm-specific threshold:
     - Sequential-phragmen: 30,000ms (30s)
     - Parallel-phragmen: 15,000ms (15s)
     - Multi-phase: 45,000ms (45s)
   - `threshold_passed` must be `true` if `timing_ms <= threshold_ms`, `false` otherwise
   - Benchmark test fails if `threshold_passed` is `false`

### Memory Validation

1. **Memory Usage**:
   - `memory_mb` must be non-negative
   - `memory_mb = 0` indicates measurement unavailable (graceful degradation)
   - `memory_measurement_available` must be `false` if `memory_mb = 0` due to unsupported platform
   - Represents peak memory usage during benchmark execution

### Metadata Validation

1. **Required Fields**:
   - `benchmark_name` must be non-empty (typically "polkadot_mainnet")
   - `candidate_count` must be positive (typically ~300-400 for Polkadot mainnet)
   - `nominator_count` must be positive (typically ~20k-30k for Polkadot mainnet)
   - `algorithm` must be one of: "sequential-phragmen", "parallel-phragmen", "multi-phase"
   - `block_number` must be a valid Polkadot mainnet block number (u64)
   - `chain` must be "polkadot"
   - `timestamp` must be valid ISO 8601 datetime string

2. **Block Number Validation**:
   - `block_number` should be within last 30 days by default
   - May be overridden for specific regression testing scenarios
   - Archive node endpoints required for historical blocks (>30 days old)

3. **Scale Validation**:
   - `candidate_count` should be ~300-400 for Polkadot mainnet scale
   - `nominator_count` should be ~20k-30k for Polkadot mainnet scale
   - Warnings may be issued if counts are outside expected ranges

## Example Output

### Single Iteration Benchmark (Success)

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
    "rpc_endpoint": "https://polkadot.api.onfinality.io/public",
    "iterations": 1,
    "threshold_ms": 30000,
    "threshold_passed": true,
    "hardware_info": {
      "cpu_cores": 8,
      "ram_gb": 16,
      "os": "macOS 14.0",
      "cpu_model": "Apple M1"
    },
    "environment_info": {
      "rust_version": "1.70.0",
      "cargo_version": "1.70.0",
      "test_runner": "cargo test"
    },
    "timestamp": "2025-01-27T12:00:00Z",
    "memory_measurement_available": true
  }
}
```

### Single Iteration Benchmark (Threshold Exceeded)

```json
{
  "timing_ms": 35234,
  "memory_mb": 398,
  "metadata": {
    "benchmark_name": "polkadot_mainnet",
    "candidate_count": 401,
    "nominator_count": 28901,
    "algorithm": "sequential-phragmen",
    "block_number": 20000001,
    "chain": "polkadot",
    "rpc_endpoint": "https://polkadot.api.onfinality.io/public",
    "iterations": 1,
    "threshold_ms": 30000,
    "threshold_passed": false,
    "hardware_info": {
      "cpu_cores": 4,
      "ram_gb": 8,
      "os": "Linux",
      "cpu_model": "Intel Core i5"
    },
    "timestamp": "2025-01-27T12:05:00Z",
    "memory_measurement_available": true
  }
}
```

### Single Iteration Benchmark (Memory Measurement Unavailable)

```json
{
  "timing_ms": 14234,
  "memory_mb": 0,
  "metadata": {
    "benchmark_name": "polkadot_mainnet",
    "candidate_count": 378,
    "nominator_count": 22123,
    "algorithm": "parallel-phragmen",
    "block_number": 20000002,
    "chain": "polkadot",
    "rpc_endpoint": "https://rpc.polkadot.io",
    "iterations": 1,
    "threshold_ms": 15000,
    "threshold_passed": true,
    "hardware_info": {
      "cpu_cores": 8,
      "ram_gb": 16,
      "os": "FreeBSD"
    },
    "timestamp": "2025-01-27T12:10:00Z",
    "memory_measurement_available": false
  }
}
```

## Performance Thresholds

Algorithm-specific performance thresholds for Polkadot mainnet scale (~300-400 candidates, ~20k-30k nominators):

| Algorithm | Threshold | Rationale |
|-----------|-----------|-----------|
| Sequential-phragmen | < 30s (30,000ms) | Slower but simpler algorithm, more predictable |
| Parallel-phragmen | < 15s (15,000ms) | Faster parallel algorithm, optimized for performance |
| Multi-phase | < 45s (45,000ms) | Most complex algorithm, multiple phases increase execution time |

**Note**: Thresholds are based on Polkadot mainnet scale characteristics. Actual performance may vary based on:
- Hardware specifications
- Voting pattern density
- Stake distribution
- System load

## Output Locations

### JSON Files

**Directory**: `tests/fixtures/benchmarks/`

**Filename Pattern**: `polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json`

**Example**: `polkadot_mainnet_20000000_sequential-phragmen_2025-01-27T12:00:00Z.json`

**Format**: Pretty-printed JSON with 2-space indentation

### Markdown Documentation

**File**: `PERFORMANCE_BENCHMARKS.md` (repository root)

**Format**: Appended summary table with latest results:

```markdown
## Polkadot Mainnet Benchmark Results

| Date | Block | Algorithm | Execution Time (ms) | Memory (MB) | Candidates | Nominators | Threshold Passed |
|------|-------|-----------|---------------------|-------------|------------|------------|------------------|
| 2025-01-27 | 20000000 | sequential-phragmen | 15234 | 342 | 387 | 23456 | ✅ |
| 2025-01-27 | 20000001 | sequential-phragmen | 35234 | 398 | 401 | 28901 | ❌ |
| 2025-01-27 | 20000002 | parallel-phragmen | 14234 | 0 (N/A) | 378 | 22123 | ✅ |
```

## Error Handling

### RPC Fetch Failures

If RPC endpoint fails after retries:

```json
{
  "error": "Failed to fetch Polkadot mainnet data after 3 retry attempts",
  "error_details": "Last error: Connection timeout",
  "rpc_endpoint": "https://polkadot.api.onfinality.io/public",
  "block_number": 20000000,
  "suggested_endpoints": [
    "https://rpc.polkadot.io",
    "https://polkadot-rpc.dwellir.com"
  ],
  "timestamp": "2025-01-27T12:00:00Z"
}
```

### Memory Measurement Failures

If memory measurement fails on unsupported platform:

- `memory_mb` set to `0`
- `memory_measurement_available` set to `false`
- Warning message included in test output
- Benchmark continues and completes successfully

### Threshold Violations

If execution time exceeds threshold:

- `threshold_passed` set to `false`
- Benchmark test fails with clear error message
- Results still written to JSON file and markdown documentation
- Error message includes threshold value and actual execution time

## Comparison and Regression Detection

Benchmark output can be compared across runs to detect performance regressions:

1. **Execution Time Trends**:
   - Compare `timing_ms` across different runs
   - Detect if execution time increases beyond acceptable threshold (e.g., 10% increase)
   - Use `mean_time_ms` and `std_dev_ms` for statistical analysis

2. **Memory Usage Trends**:
   - Compare `memory_mb` across different runs
   - Detect memory leaks or excessive memory growth
   - Account for `memory_measurement_available` flag

3. **Threshold Compliance**:
   - Track `threshold_passed` over time
   - Identify algorithms that consistently exceed thresholds
   - Correlate with hardware specifications

4. **Block Number Comparison**:
   - Compare performance across different block numbers
   - Identify performance variations based on chain state
   - Track performance trends over time

## Integration Points

### Test Framework Integration

- Benchmark tests use `#[test]` and `#[ignore]` attributes
- Run with `cargo test -- --ignored` flag
- Results validated against thresholds in test assertions
- JSON output written to `tests/fixtures/benchmarks/` directory

### CI/CD Integration

- Benchmarks can be run in CI/CD pipelines
- JSON output parsed for threshold validation
- Markdown documentation updated automatically
- Performance regressions detected via threshold violations

### Documentation Integration

- Results appended to `PERFORMANCE_BENCHMARKS.md`
- Hardware specifications included for context
- Trend analysis enabled via historical data
- Human-readable format for manual review

