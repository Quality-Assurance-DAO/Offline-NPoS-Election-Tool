# Data Model: Comprehensive Test Enhancement

**Date**: 2025-01-27  
**Feature**: 002-comprehensive-testing

## Overview

This document defines the data models for the comprehensive test enhancement feature. All entities, their fields, relationships, validation rules, and state transitions are documented here.

## Core Entities

### 1. TestFixture

Represents a specific election dataset used for testing, including input data, configuration, and expected results.

**Fields**:
- `metadata: TestFixtureMetadata` - Metadata about the test scenario
- `input: ElectionData` - Input election data (candidates, nominators, configuration)
- `expected_result: Option<ElectionResult>` - Expected election result (for regression tests)

**Validation Rules**:
- Must contain valid `ElectionData` structure
- Expected result must match `ElectionData` structure if provided
- Metadata must include test name and description

**Relationships**:
- Contains `ElectionData` (reuses existing model)
- Contains optional `ElectionResult` (reuses existing model)
- Referenced by `RegressionTest` and `EdgeCaseTest`

**State Transitions**:
- **Created**: Test fixture JSON file created
- **Loaded**: Test fixture loaded into test framework
- **Validated**: Input data validated against schema
- **Executed**: Election run with fixture data
- **Compared**: Results compared to expected results (if provided)

---

### 2. TestFixtureMetadata

Metadata about a test fixture, including test scenario information and creation details.

**Fields**:
- `test_name: String` - Unique identifier for the test scenario
- `description: String` - Human-readable description of what the test validates
- `created: DateTime<Utc>` - Timestamp when fixture was created
- `algorithm: AlgorithmType` - Election algorithm used (sequential-phragmen, parallel-phragmen, multi-phase)
- `category: TestCategory` - Test category (edge_case, performance, regression, chain_snapshot)
- `tags: Vec<String>` - Optional tags for test organization (e.g., "zero_candidates", "large_scale")

**Validation Rules**:
- `test_name` must be unique within category
- `description` must be non-empty
- `algorithm` must be valid `AlgorithmType`
- `category` must be valid `TestCategory`

**Relationships**:
- Part of `TestFixture`
- Used for test discovery and organization

---

### 3. ChainSnapshot

Represents election data captured from a specific block on a real Substrate chain, stored as JSON with metadata for regression testing.

**Fields**:
- `metadata: ChainSnapshotMetadata` - Chain and block information
- `election_data: ElectionData` - Election data fetched from chain at specified block
- `expected_result: ElectionResult` - Actual on-chain election result for comparison

**Validation Rules**:
- `metadata.block_number` must be valid historical block
- `metadata.chain` must be valid chain identifier
- `election_data` must match structure from chain RPC
- `expected_result` must contain exact validator selections and stake allocations from on-chain

**Relationships**:
- Contains `ElectionData` (reuses existing model)
- Contains `ElectionResult` (reuses existing model)
- Referenced by `ChainSnapshotTest`

**State Transitions**:
- **Fetched**: Data fetched from chain RPC endpoint
- **Stored**: Snapshot saved as JSON file
- **Loaded**: Snapshot loaded from file for testing
- **Validated**: Snapshot data validated against on-chain results
- **Compared**: Simulated results compared to expected on-chain results

---

### 4. ChainSnapshotMetadata

Metadata about a chain snapshot, including chain identifier, block number, and RPC endpoint information.

**Fields**:
- `chain: String` - Chain identifier (e.g., "polkadot", "kusama")
- `block_number: u64` - Block number at which snapshot was taken
- `timestamp: DateTime<Utc>` - Timestamp when snapshot was created
- `rpc_endpoint: String` - RPC endpoint URL used to fetch data
- `expected_validators: Vec<AccountId>` - List of validator account IDs selected on-chain
- `expected_stake_allocations: HashMap<AccountId, HashMap<AccountId, Balance>>` - On-chain stake allocations (nominator -> validator -> amount)

**Validation Rules**:
- `chain` must be non-empty and valid chain identifier
- `block_number` must be valid historical block number
- `rpc_endpoint` must be valid URL
- `expected_validators` must match actual on-chain validator set
- `expected_stake_allocations` must match actual on-chain stake distribution

**Relationships**:
- Part of `ChainSnapshot`
- Used for result comparison and validation

---

### 5. PerformanceBenchmark

Represents a performance test that measures execution time, memory usage, and other metrics for election execution.

**Fields**:
- `metadata: BenchmarkMetadata` - Benchmark configuration and metadata
- `input: ElectionData` - Input election data for benchmark
- `results: BenchmarkResults` - Performance measurement results

**Validation Rules**:
- Input data must be valid `ElectionData`
- Results must include timing and memory measurements
- Metadata must specify performance targets/thresholds

**Relationships**:
- Contains `ElectionData` (reuses existing model)
- Contains `BenchmarkResults`
- Referenced by `PerformanceTest`

**State Transitions**:
- **Configured**: Benchmark test configured with input data
- **Warmed**: Benchmark warmup phase completed
- **Executed**: Benchmark execution completed
- **Measured**: Performance metrics collected
- **Reported**: Results output in structured JSON format

---

### 6. BenchmarkMetadata

Metadata about a performance benchmark, including dataset characteristics and performance targets.

**Fields**:
- `benchmark_name: String` - Unique identifier for the benchmark
- `description: String` - Description of what the benchmark measures
- `candidate_count: usize` - Number of validator candidates in dataset
- `nominator_count: usize` - Number of nominators in dataset
- `algorithm: AlgorithmType` - Election algorithm being benchmarked
- `target_time_ms: Option<u64>` - Target execution time in milliseconds (e.g., 60000 for 60s)
- `target_memory_mb: Option<u64>` - Target memory usage in MB (e.g., 8192 for 8GB)

**Validation Rules**:
- `benchmark_name` must be unique
- `candidate_count` and `nominator_count` must be positive
- `target_time_ms` and `target_memory_mb` are optional but recommended

**Relationships**:
- Part of `PerformanceBenchmark`
- Used for performance target validation

---

### 7. BenchmarkResults

Results from a performance benchmark execution, including timing, memory usage, and metadata.

**Fields**:
- `execution_time_ms: u64` - Total execution time in milliseconds
- `memory_peak_mb: u64` - Peak memory usage in MB
- `memory_final_mb: u64` - Final memory usage after execution
- `iterations: usize` - Number of iterations run (for statistical analysis)
- `mean_time_ms: Option<f64>` - Mean execution time (if multiple iterations)
- `std_dev_ms: Option<f64>` - Standard deviation of execution times
- `metadata: HashMap<String, String>` - Additional metadata (e.g., hardware info, Rust version)

**Validation Rules**:
- All timing values must be non-negative
- All memory values must be non-negative
- `mean_time_ms` and `std_dev_ms` must be present if `iterations > 1`

**Relationships**:
- Part of `PerformanceBenchmark`
- Used for performance comparison and regression detection

**Output Format**:
Structured JSON:
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

---

### 8. EdgeCaseScenario

Represents a test scenario that exercises boundary conditions, unusual inputs, or error conditions.

**Fields**:
- `scenario_name: String` - Unique identifier for the scenario
- `description: String` - Description of the edge case being tested
- `input: ElectionData` - Input data representing the edge case
- `expected_behavior: ExpectedBehavior` - Expected system behavior (success with specific result, or error with specific message)
- `tags: Vec<String>` - Tags for categorization (e.g., "zero_candidates", "malformed_data", "boundary_condition")

**Validation Rules**:
- `scenario_name` must be unique
- `description` must clearly explain the edge case
- `expected_behavior` must specify either success criteria or error message pattern

**Relationships**:
- Contains `ElectionData` (reuses existing model)
- Contains `ExpectedBehavior`
- Referenced by `EdgeCaseTest`

---

### 9. ExpectedBehavior

Defines the expected system behavior for an edge case test scenario.

**Fields**:
- `should_succeed: bool` - Whether the election should complete successfully
- `expected_result: Option<ElectionResult>` - Expected result if successful
- `expected_error: Option<ErrorPattern>` - Expected error pattern if failure
- `error_message_contains: Option<Vec<String>>` - Substrings that must appear in error message

**Validation Rules**:
- Either `should_succeed` is true with `expected_result`, or false with `expected_error`
- `error_message_contains` must be provided if `expected_error` is present

**Relationships**:
- Part of `EdgeCaseScenario`
- Used for test validation

---

### 10. RegressionTest

Represents a test that validates election results remain consistent across code changes.

**Fields**:
- `test_fixture: TestFixture` - Test fixture with input and expected results
- `last_updated: DateTime<Utc>` - Timestamp when test was last updated
- `version_introduced: String` - Tool version when test was introduced
- `baseline_result: ElectionResult` - Baseline result from when test was created

**Validation Rules**:
- `test_fixture` must include expected result
- `baseline_result` must match `test_fixture.expected_result` initially
- Test should fail if results differ from baseline (unless intentional change)

**Relationships**:
- Contains `TestFixture` (reuses existing model)
- Contains `ElectionResult` (reuses existing model)
- Used for regression detection

---

## Test Categories

### TestCategory (Enum)

Categories for organizing tests:

- `EdgeCase` - Tests for boundary conditions and unusual inputs
- `Performance` - Tests measuring execution time and memory usage
- `Regression` - Tests ensuring results remain consistent across changes
- `ChainSnapshot` - Tests validating accuracy against on-chain results
- `Integration` - End-to-end integration tests

---

## File Structure

### Test Fixture File Format

```json
{
  "metadata": {
    "test_name": "single_candidate_single_nominator",
    "description": "Edge case: exactly one candidate and one nominator",
    "created": "2025-01-27T10:00:00Z",
    "algorithm": "sequential-phragmen",
    "category": "edge_case",
    "tags": ["zero_candidates", "single_nominator"]
  },
  "input": {
    "candidates": [...],
    "nominators": [...],
    "config": {
      "active_set_size": 1,
      "algorithm": "sequential-phragmen"
    }
  },
  "expected_result": {
    "selected_validators": [...],
    "stake_allocations": {...}
  }
}
```

### Chain Snapshot File Format

```json
{
  "metadata": {
    "chain": "polkadot",
    "block_number": 12345678,
    "timestamp": "2024-01-15T10:30:00Z",
    "rpc_endpoint": "wss://rpc.polkadot.io",
    "expected_validators": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
    "expected_stake_allocations": {
      "nominator_id": {
        "validator_id": 1000000000
      }
    }
  },
  "election_data": {
    "candidates": [...],
    "nominators": [...]
  }
}
```

---

## Validation Rules Summary

1. **Test Fixtures**: Must contain valid `ElectionData` and optional `ElectionResult`
2. **Chain Snapshots**: Must include chain metadata and match on-chain results exactly
3. **Performance Benchmarks**: Must include timing and memory measurements
4. **Edge Case Scenarios**: Must specify expected behavior (success or error)
5. **Regression Tests**: Must include baseline results for comparison
6. **All Test Data**: Must be stored as version-controlled JSON files

---

## State Transitions Summary

- **Test Fixtures**: Created → Loaded → Validated → Executed → Compared
- **Chain Snapshots**: Fetched → Stored → Loaded → Validated → Compared
- **Performance Benchmarks**: Configured → Warmed → Executed → Measured → Reported
- **Edge Case Scenarios**: Created → Loaded → Executed → Validated
- **Regression Tests**: Created → Executed → Compared → (Pass/Fail)

