# Research: Comprehensive Test Enhancement

**Date**: 2025-01-27  
**Feature**: 002-comprehensive-testing

## Overview

This document consolidates research findings for technical decisions required to implement comprehensive test enhancements. All items marked as "NEEDS CLARIFICATION" in the Technical Context section of `plan.md` are resolved here.

## Technical Decisions

### 1. Rust Test Organization and Structure

**Decision**: Use standard Rust test organization with `tests/` directory for integration tests and `#[cfg(test)]` modules for unit tests. Organize integration tests by category (edge_cases, performance, chain_snapshots, regression).

**Rationale**:
- Follows Rust conventions: `cargo test` automatically discovers tests in `tests/` directory
- Integration tests in `tests/` are compiled as separate binaries, allowing better isolation
- Unit tests in `#[cfg(test)]` modules can access private functions and test implementation details
- Category-based subdirectories improve test discoverability and organization
- Standard approach ensures compatibility with CI/CD systems and IDE tooling

**Alternatives Considered**:
- **Single test file**: Would become unmanageable with 50+ regression tests and multiple test categories
- **Test framework crates (e.g., `rstest`)**: Adds dependencies, standard `cargo test` is sufficient for needs
- **Separate test binaries**: Overkill for current scope, standard organization sufficient

**Implementation Details**:
- `tests/unit/` - Unit tests for individual components (algorithms, models, input parsing)
- `tests/integration/edge_cases/` - Edge case scenarios (zero candidates, malformed data, boundary conditions)
- `tests/integration/performance/` - Performance benchmarks (large datasets, timing measurements)
- `tests/integration/chain_snapshots/` - Chain snapshot regression tests (RPC data, on-chain validation)
- `tests/integration/regression/` - Regression test fixtures (known inputs/outputs, result consistency)

---

### 2. Performance Benchmarking Approach

**Decision**: Use `criterion` crate for performance benchmarks with structured JSON output, supplemented by custom integration tests for large-scale performance validation.

**Rationale**:
- `criterion` provides statistical analysis, regression detection, and HTML reports
- Supports JSON output for automated comparison and CI integration
- Handles warmup, iteration, and statistical analysis automatically
- Well-maintained and widely used in Rust ecosystem
- Custom integration tests needed for very large datasets (10k+ candidates) that may exceed benchmark time limits

**Alternatives Considered**:
- **Custom timing in integration tests**: Less statistical rigor, no regression detection
- **`cargo bench` only**: Limited to smaller datasets, less flexible for very large scale tests
- **External benchmarking tools**: Adds complexity, `criterion` integrates well with Rust tooling

**Implementation Details**:
- Use `criterion` for standard benchmarks (1k-5k candidates)
- Custom integration tests for very large datasets (10k+ candidates) with timing and memory measurement
- Output structured JSON: `{ "timing_ms": 1234, "memory_mb": 256, "metadata": {...} }`
- Store benchmark results in `tests/fixtures/benchmarks/` for comparison over time
- Performance targets: 1k candidates/10k nominators < 60s, 5k/50k < 5min, 10k/100k no OOM

---

### 3. RPC Retry Pattern with Exponential Backoff

**Decision**: Implement exponential backoff retry logic using `tokio::time::sleep` with configurable retry attempts (default 3) and exponential backoff (1s, 2s, 4s).

**Rationale**:
- Handles transient network failures gracefully (FR-021)
- Exponential backoff prevents overwhelming RPC endpoints during outages
- Standard pattern for network request retries
- `tokio::time::sleep` is async-friendly and doesn't block the runtime
- Configurable retry count allows adjustment for different RPC endpoints

**Alternatives Considered**:
- **Fixed retry interval**: Less efficient, doesn't account for temporary overload
- **External retry crate (e.g., `tokio-retry`)**: Adds dependency, simple implementation sufficient
- **No retries**: Would cause test failures on transient network issues, poor developer experience

**Implementation Details**:
- Retry up to 3 attempts with exponential backoff: `delay = 2^attempt seconds`
- First retry after 1s, second after 2s, third after 4s
- On all retries exhausted, mark test as skipped with clear reason: "RPC endpoint unavailable after 3 retries"
- Use `#[ignore]` attribute or custom test skipping mechanism for failed RPC tests
- Log retry attempts for debugging: "RPC call failed, retrying in 1s (attempt 1/3)"

---

### 4. Chain Snapshot Data Structure and Storage

**Decision**: Store chain snapshot data as version-controlled JSON files in `tests/fixtures/chain_snapshots/{chain}/{block_number}.json` with metadata header including chain identifier, block number, timestamp, and expected on-chain election results.

**Rationale**:
- Version-controlled JSON files ensure reproducibility and avoid repeated RPC calls
- Directory structure by chain improves organization and discoverability
- JSON format is human-readable and easy to validate
- Metadata header enables test framework to validate expected results automatically
- Storing snapshots prevents test failures due to RPC unavailability

**Alternatives Considered**:
- **Fetch from RPC on every test run**: Slow, unreliable, causes test failures on network issues
- **Binary format**: Less human-readable, harder to review and validate
- **Database storage**: Overkill, adds complexity, JSON files sufficient

**Implementation Details**:
- File structure:
  ```json
  {
    "metadata": {
      "chain": "polkadot",
      "block_number": 12345678,
      "timestamp": "2024-01-15T10:30:00Z",
      "rpc_endpoint": "wss://rpc.polkadot.io",
      "expected_validators": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
      "expected_stake_allocations": {...}
    },
    "election_data": {
      "candidates": [...],
      "nominators": [...]
    }
  }
  ```
- Store snapshots for at least 3 chains (Polkadot, Kusama, and one other)
- Include at least 10 historical blocks per chain
- Update snapshots periodically or when chain upgrades occur

---

### 5. Regression Test Fixture Management

**Decision**: Store regression test fixtures as version-controlled JSON files in `tests/fixtures/regression/` with both input data and expected results, organized by test scenario name.

**Rationale**:
- Version-controlled fixtures preserve historical test cases (FR-015)
- JSON format enables easy review and modification
- Storing expected results enables automatic comparison and change detection
- Organized by scenario name improves discoverability
- Can be used for both manual review and automated testing

**Alternatives Considered**:
- **Hardcoded test data**: Harder to review, less flexible for complex scenarios
- **Separate expected results files**: More files to manage, JSON structure allows both together
- **Database storage**: Overkill, version control provides history tracking

**Implementation Details**:
- File structure:
  ```json
  {
    "metadata": {
      "test_name": "single_candidate_single_nominator",
      "description": "Edge case: exactly one candidate and one nominator",
      "created": "2025-01-27",
      "algorithm": "sequential-phragmen"
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
- At least 50 regression test fixtures covering edge cases, normal scenarios, and boundary conditions
- Test framework compares actual results to expected results and reports differences
- Fixtures can be updated when intentional algorithm changes occur

---

### 6. Memory Leak Detection Approach

**Decision**: Use custom integration test that runs 100+ consecutive elections and monitors memory usage using `std::alloc::System` or `jemalloc` statistics, detecting increasing memory trends.

**Rationale**:
- Memory leaks can cause OOM failures in production with large datasets
- Running consecutive elections exercises the full election lifecycle
- Monitoring memory usage detects gradual leaks that single-run tests miss
- Standard approach for detecting resource leaks in long-running scenarios

**Alternatives Considered**:
- **Single election test**: Would miss gradual memory leaks
- **External memory profiler**: Adds complexity, basic monitoring sufficient for detection
- **No memory leak testing**: Would miss critical production issues

**Implementation Details**:
- Run 100 consecutive elections with same or varying datasets
- Measure memory usage before and after each election
- Detect if memory usage increases linearly (indicates leak) vs. stable (no leak)
- Use `std::alloc::System` or enable `jemalloc` feature for better memory statistics
- Test should pass if memory usage remains stable or decreases

---

### 7. Concurrent Election Execution Testing

**Decision**: Use `tokio::task::spawn` or `std::thread::spawn` to run multiple independent elections in parallel, validating that concurrent execution doesn't cause race conditions or incorrect results.

**Rationale**:
- Validates thread-safety of election engine (FR-020)
- Ensures algorithm implementations are safe for concurrent use
- Tests realistic scenarios where multiple elections might run simultaneously
- Uses standard Rust concurrency primitives

**Alternatives Considered**:
- **Sequential execution only**: Would miss concurrency bugs
- **Complex concurrency framework**: Overkill, standard primitives sufficient

**Implementation Details**:
- Spawn 4-8 parallel tasks, each running independent election with different datasets
- Verify all elections complete successfully
- Verify results are deterministic and match sequential execution
- Test with different algorithms to ensure thread-safety across implementations
- Use `tokio::task::spawn` for async contexts, `std::thread::spawn` for sync contexts

---

## Dependencies Summary

**New Dependencies**:
- `criterion` - Performance benchmarking framework (dev-dependency)
- `tokio` - Already present, used for async RPC retries and concurrent tests

**Existing Dependencies Used**:
- `serde`, `serde_json` - JSON serialization for test fixtures
- `jsonrpsee` - RPC client for chain snapshot data (already present)
- Built-in `cargo test` - Test framework

---

## Open Questions Resolved

✅ **Test Organization**: Standard Rust test structure with category-based subdirectories  
✅ **Performance Benchmarking**: `criterion` for standard benchmarks, custom tests for very large datasets  
✅ **RPC Retry Logic**: Exponential backoff with 3 attempts, 1s/2s/4s delays  
✅ **Chain Snapshot Storage**: Version-controlled JSON files in `tests/fixtures/chain_snapshots/`  
✅ **Regression Fixtures**: Version-controlled JSON files in `tests/fixtures/regression/` with input and expected results  
✅ **Memory Leak Detection**: Custom test running 100+ consecutive elections monitoring memory usage  
✅ **Concurrent Execution**: Standard Rust concurrency primitives (`tokio::task::spawn` or `std::thread::spawn`)

All technical unknowns from the Technical Context section have been resolved.

