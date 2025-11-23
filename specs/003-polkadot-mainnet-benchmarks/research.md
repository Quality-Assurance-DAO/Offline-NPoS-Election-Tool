# Research: Polkadot Mainnet Performance Benchmarks

**Date**: 2025-01-27  
**Feature**: 003-polkadot-mainnet-benchmarks  
**Status**: Complete

## Overview

This document consolidates research findings for implementing performance benchmarks with real Polkadot mainnet data. Research addresses technical unknowns identified in the implementation plan, focusing on memory measurement implementation, RPC retry patterns, and benchmark result storage.

## Research Tasks

### 1. Memory Measurement Implementation

**Question**: How should memory usage be measured on Linux, macOS, and Windows with graceful degradation on unsupported platforms?

**Decision**: Use platform-specific APIs with a trait-based abstraction for cross-platform support.

**Rationale**:
- Rust's standard library doesn't provide direct memory measurement APIs
- Platform-specific approaches are necessary for accurate measurement
- Graceful degradation ensures benchmarks complete successfully even when measurement fails

**Implementation Approach**:
- **Linux**: Read `/proc/self/status` to get `VmPeak` (peak virtual memory) and `VmRSS` (resident set size)
- **macOS**: Use `mach_task_basic_info` via `libc` to get `resident_size` and `virtual_size`
- **Windows**: Use `GetProcessMemoryInfo` from `winapi` crate to get `PeakWorkingSetSize` and `WorkingSetSize`
- **Unsupported platforms**: Return `None` or `0` with a warning message

**Alternatives Considered**:
- **jemalloc statistics**: Requires linking against jemalloc, adds complexity, not available on all platforms
- **System allocator statistics**: Limited accuracy, doesn't capture peak memory reliably
- **External tools** (e.g., `/usr/bin/time`): Requires subprocess execution, harder to integrate into test framework

**Dependencies**:
- `libc` crate for macOS system calls
- `winapi` crate for Windows APIs (or use `windows-sys` for newer Windows API bindings)
- Conditional compilation with `#[cfg(target_os = "...")]` attributes

**Code Structure**:
```rust
pub trait MemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError>;
}

#[cfg(target_os = "linux")]
impl MemoryMeasurer for LinuxMemoryMeasurer { ... }

#[cfg(target_os = "macos")]
impl MemoryMeasurer for MacOSMemoryMeasurer { ... }

#[cfg(target_os = "windows")]
impl MemoryMeasurer for WindowsMemoryMeasurer { ... }

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
impl MemoryMeasurer for UnsupportedMemoryMeasurer {
    fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError> {
        Err(MemoryMeasurementError::UnsupportedPlatform)
    }
}
```

### 2. RPC Retry Logic and Error Handling

**Question**: How should RPC endpoint failures be handled with retry logic and timeout strategy?

**Decision**: Use existing `tests/common/rpc_retry.rs` utilities with exponential backoff (1s, 2s, 4s delays, up to 3 retries) and 30s timeout per request.

**Rationale**:
- Retry logic already exists in `tests/common/rpc_retry.rs` with `retry_with_backoff` function
- RPC client (`src/input/rpc.rs`) already has 30s timeout configured
- Exponential backoff reduces load on RPC endpoints and handles transient failures
- 3 retries provide good balance between reliability and execution time

**Implementation Details**:
- Use existing `retry_with_backoff` function from `tests/common/rpc_retry.rs`
- Configure with `max_attempts: 3`, `initial_delay: Duration::from_secs(1)`
- RPC client timeout already set to 30s in `RpcLoader::new()`
- On final failure, provide clear error message with alternative endpoint suggestions

**Error Message Format**:
```
Failed to fetch Polkadot mainnet data after 3 retry attempts.
Last error: [error details]
Suggested alternative RPC endpoints:
- https://rpc.polkadot.io
- https://polkadot.api.onfinality.io/public
- https://polkadot-rpc.dwellir.com
```

**Alternatives Considered**:
- **Linear backoff**: Less effective for transient network issues
- **More retries** (5+): Increases execution time, may mask persistent failures
- **Shorter timeout** (<30s): May fail on slow but valid responses

### 3. Benchmark Result Storage Format

**Question**: What format and location should be used for documenting benchmark results?

**Decision**: Structured JSON files in `tests/fixtures/benchmarks/` directory plus markdown performance documentation file.

**Rationale**:
- JSON format enables programmatic comparison and CI integration
- Markdown format provides human-readable documentation
- `tests/fixtures/benchmarks/` directory aligns with existing test fixture structure
- Separation of concerns: JSON for automation, markdown for documentation

**JSON File Format**:
- Filename pattern: `polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json`
- Schema matches existing `BenchmarkResults` struct from `tests/common/models.rs`
- Includes execution time, memory usage, candidate count, nominator count, block number, algorithm

**Markdown Documentation**:
- Append results to `PERFORMANCE_BENCHMARKS.md` file
- Include summary table with latest results
- Include hardware specifications and environment details
- Enable trend tracking over time

**Alternatives Considered**:
- **Database storage**: Overkill for benchmark results, adds complexity
- **Single JSON file**: Harder to track historical results, file size grows over time
- **Separate markdown per benchmark**: Too many files, harder to compare results

### 4. Block Number Selection Strategy

**Question**: How should block numbers be selected for benchmarks (fixed, latest, or configurable)?

**Decision**: Fixed recent block number (within last 30 days) with optional override parameter.

**Rationale**:
- Fixed block ensures reproducible benchmarks
- Recent block (within 30 days) ensures data is representative of current mainnet state
- Optional override allows testing with specific blocks for regression testing
- Archive node endpoints required for historical blocks (already documented)

**Implementation**:
- Default: Calculate block number as `latest_block - (30 days * blocks_per_day)`
- Polkadot block time: ~6 seconds, so ~30 days ≈ 432,000 blocks
- Allow override via test parameter or CLI flag: `--block-number <NUMBER>`
- Validate block number is not too old (>30 days) unless explicitly overridden

**Alternatives Considered**:
- **Always latest block**: Not reproducible, results vary between runs
- **Fixed historical block**: May become outdated, doesn't reflect current mainnet state
- **Random recent block**: Less reproducible, harder to track trends

### 5. Performance Threshold Validation

**Question**: How should algorithm-specific performance thresholds be validated?

**Decision**: Validate execution times against algorithm-specific thresholds in benchmark tests, fail test if threshold exceeded.

**Rationale**:
- Algorithm-specific thresholds account for different algorithm complexities
- Sequential-phragmen is slower but simpler, parallel-phragmen is faster, multi-phase is most complex
- Thresholds based on Polkadot mainnet scale (~300-400 candidates, ~20k-30k nominators)
- Test failures provide clear feedback on performance regressions

**Threshold Values**:
- Sequential-phragmen: < 30s
- Parallel-phragmen: < 15s
- Multi-phase: < 45s

**Implementation**:
- Add threshold validation to benchmark test assertions
- Include threshold in benchmark metadata
- Allow threshold override for different hardware configurations
- Report threshold violations clearly in test output

**Alternatives Considered**:
- **Single threshold for all algorithms**: Doesn't account for algorithm complexity differences
- **No thresholds**: No automated regression detection
- **Warnings instead of failures**: May miss regressions, less strict enforcement

### 6. Benchmark Test Integration

**Question**: How should benchmarks be integrated into the test suite (always run, ignored by default, or separate command)?

**Decision**: Benchmarks marked as `#[ignore]` by default, requiring `--ignored` flag to run, due to network dependencies.

**Rationale**:
- Network dependencies make benchmarks unsuitable for standard test runs
- `#[ignore]` allows benchmarks to be run on-demand without affecting CI/CD
- Can be enabled in CI/CD with `cargo test -- --ignored` flag
- Prevents accidental network calls during development

**Implementation**:
- Mark benchmark tests with `#[test]` and `#[ignore]` attributes
- Document in test comments that network access is required
- Provide instructions for running benchmarks in `PERFORMANCE_BENCHMARKS.md`
- Consider separate `cargo bench` command for performance benchmarks (future enhancement)

**Alternatives Considered**:
- **Always run**: May fail in environments without network access, slows down test suite
- **Separate binary**: Adds complexity, harder to integrate with existing test infrastructure
- **Feature flag**: More complex than `#[ignore]`, requires build-time configuration

## Dependencies Summary

**New Dependencies**:
- `libc` crate (for macOS memory measurement) - version 0.2 or later
- `winapi` crate (for Windows memory measurement) - version 0.3 or later, features: `winbase`, `psapi`

**Existing Dependencies** (already in `Cargo.toml`):
- `jsonrpsee` - RPC client (already configured with 30s timeout)
- `tokio` - Async runtime (already included)
- `serde`, `serde_json` - JSON serialization (already included)
- `chrono` - Timestamp handling (already included)

**No New Dependencies Required**:
- RPC retry logic already exists in `tests/common/rpc_retry.rs`
- Benchmark utilities already exist in `tests/common/benchmark_utils.rs`
- Test models already exist in `tests/common/models.rs`

## Open Questions Resolved

✅ **Memory Measurement**: Platform-specific APIs with trait-based abstraction  
✅ **RPC Retry Logic**: Use existing `retry_with_backoff` with exponential backoff  
✅ **Benchmark Storage**: JSON files in `tests/fixtures/benchmarks/` plus markdown documentation  
✅ **Block Number Selection**: Fixed recent block (within 30 days) with optional override  
✅ **Performance Thresholds**: Algorithm-specific thresholds with test validation  
✅ **Test Integration**: `#[ignore]` attribute for network-dependent benchmarks  

All technical unknowns from the Technical Context section have been resolved.

## Implementation Notes

1. **Memory Measurement**: Start with Linux implementation, then add macOS and Windows support. Use conditional compilation to avoid platform-specific code in unsupported environments.

2. **RPC Retry**: Leverage existing `retry_with_backoff` function, but may need to enhance error messages with alternative endpoint suggestions.

3. **Benchmark Results**: Create `tests/fixtures/benchmarks/` directory structure, ensure JSON schema matches `BenchmarkResults` struct.

4. **Performance Documentation**: Update `PERFORMANCE_BENCHMARKS.md` with new benchmark results section, include hardware specifications and environment details.

5. **Test Structure**: Create new integration test file `tests/integration/performance/test_polkadot_mainnet.rs` following existing performance test patterns.

