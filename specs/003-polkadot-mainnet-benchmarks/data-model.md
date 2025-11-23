# Data Model: Polkadot Mainnet Performance Benchmarks

**Date**: 2025-01-27  
**Feature**: 003-polkadot-mainnet-benchmarks  
**Status**: Design Complete

## Overview

This document defines the data models for Polkadot mainnet performance benchmarks. The models extend existing benchmark infrastructure to support real-world data benchmarks with execution time, memory usage, and metadata tracking.

## Entities

### 1. BenchmarkResult

Represents a single performance benchmark execution with real Polkadot mainnet data.

**Fields**:
- `execution_time_ms` (u64, required): Total execution time in milliseconds
- `memory_peak_mb` (u64, required): Peak memory usage in MB during execution
- `memory_final_mb` (u64, required): Final memory usage in MB after execution
- `iterations` (usize, required): Number of iterations run (typically 1 for single-run benchmarks)
- `mean_time_ms` (Option<f64>, optional): Mean execution time if multiple iterations
- `std_dev_ms` (Option<f64>, optional): Standard deviation if multiple iterations
- `metadata` (HashMap<String, String>, required): Additional benchmark metadata

**Metadata Fields** (included in `metadata` HashMap):
- `benchmark_name` (String): Unique identifier for the benchmark (e.g., "polkadot_mainnet")
- `candidate_count` (String): Number of validator candidates (converted to string for HashMap)
- `nominator_count` (String): Number of nominators (converted to string for HashMap)
- `algorithm` (String): Election algorithm used ("sequential-phragmen", "parallel-phragmen", "multi-phase")
- `block_number` (String): Polkadot mainnet block number used (converted to string)
- `chain` (String): Chain identifier ("polkadot")
- `timestamp` (String): ISO 8601 timestamp when benchmark was run
- `rpc_endpoint` (String, optional): RPC endpoint URL used to fetch data
- `hardware_info` (String, optional): JSON string with hardware specifications
- `rust_version` (String, optional): Rust version used for benchmark

**Relationships**:
- Extends existing `BenchmarkResults` struct from `tests/common/models.rs`
- References `PolkadotMainnetSnapshot` for input data
- Used by `PerformanceMetrics` for threshold validation

**Validation Rules**:
- `execution_time_ms` must be non-negative
- `memory_peak_mb` must be non-negative (0 indicates measurement unavailable)
- `candidate_count` and `nominator_count` must be positive integers
- `algorithm` must be one of: "sequential-phragmen", "parallel-phragmen", "multi-phase"
- `block_number` must be a valid Polkadot mainnet block number
- If `iterations > 1`, `mean_time_ms` and `std_dev_ms` must be present

**State Transitions**: N/A (immutable data structure)

### 2. PolkadotMainnetSnapshot

Represents real on-chain election data fetched from a specific Polkadot mainnet block.

**Fields**:
- `election_data` (ElectionData, required): Election data fetched from chain (candidates, nominators, stakes)
- `block_number` (u64, required): Block number at which snapshot was taken
- `rpc_endpoint` (String, required): RPC endpoint URL used to fetch data
- `fetch_timestamp` (DateTime<Utc>, required): Timestamp when data was fetched
- `candidate_count` (usize, derived): Number of validator candidates (from `election_data.candidates.len()`)
- `nominator_count` (usize, derived): Number of nominators (from `election_data.nominators.len()`)

**Relationships**:
- Uses existing `ElectionData` struct from `src/models/election_data.rs`
- Referenced by `BenchmarkResult` for input data source
- May be cached in `tests/fixtures/chain_snapshots/` for reuse

**Validation Rules**:
- `block_number` must be within last 30 days (unless explicitly overridden)
- `election_data` must contain at least 1 candidate and 1 nominator
- `rpc_endpoint` must be a valid HTTP/HTTPS URL
- `candidate_count` should be ~300-400 for Polkadot mainnet scale
- `nominator_count` should be ~20k-30k for Polkadot mainnet scale

**State Transitions**: N/A (immutable snapshot)

### 3. PerformanceMetrics

Represents measured performance characteristics including execution time and memory usage.

**Fields**:
- `execution_time_ms` (u64, required): Execution time in milliseconds
- `memory_peak_mb` (u64, required): Peak memory usage in MB
- `memory_final_mb` (u64, required): Final memory usage in MB
- `algorithm` (AlgorithmType, required): Election algorithm used
- `threshold_ms` (u64, required): Algorithm-specific performance threshold in milliseconds

**Relationships**:
- Extracted from `BenchmarkResult` for threshold validation
- References `AlgorithmType` enum from `src/types.rs`

**Validation Rules**:
- `execution_time_ms` must be non-negative
- `memory_peak_mb` and `memory_final_mb` must be non-negative
- `threshold_ms` must match algorithm-specific threshold:
  - Sequential-phragmen: 30,000ms (30s)
  - Parallel-phragmen: 15,000ms (15s)
  - Multi-phase: 45,000ms (45s)
- `execution_time_ms <= threshold_ms` for benchmark to pass

**State Transitions**: N/A (immutable metrics)

### 4. BenchmarkMetadata

Extended metadata for Polkadot mainnet benchmarks, stored in JSON files.

**Fields** (extends `BenchmarkResults.metadata`):
- All fields from `BenchmarkResult.metadata` (see above)
- `hardware_info` (HardwareInfo, optional): Hardware specifications
- `environment_info` (EnvironmentInfo, optional): Environment details

**Nested Structures**:

#### HardwareInfo
- `cpu_cores` (u32, optional): Number of CPU cores
- `ram_gb` (u32, optional): Total RAM in GB
- `os` (String, optional): Operating system (e.g., "Linux", "macOS", "Windows")
- `cpu_model` (String, optional): CPU model name

#### EnvironmentInfo
- `rust_version` (String, optional): Rust version (e.g., "1.70.0")
- `cargo_version` (String, optional): Cargo version
- `test_runner` (String, optional): Test runner used (e.g., "cargo test")

**Relationships**:
- Included in `BenchmarkResult.metadata` as JSON string or separate fields
- Used for performance trend analysis and regression detection

**Validation Rules**:
- `cpu_cores` and `ram_gb` must be positive if present
- `os` must be one of: "Linux", "macOS", "Windows", or other valid OS identifier
- `rust_version` must match semantic versioning format (e.g., "1.70.0")

**State Transitions**: N/A (immutable metadata)

## Data Flow

### Benchmark Execution Flow

1. **Fetch Snapshot**:
   ```
   RPC Endpoint → PolkadotMainnetSnapshot (election_data, block_number)
   ```

2. **Execute Benchmark**:
   ```
   PolkadotMainnetSnapshot → ElectionEngine → PerformanceMetrics (execution_time_ms, memory_peak_mb)
   ```

3. **Create Result**:
   ```
   PerformanceMetrics + PolkadotMainnetSnapshot.metadata → BenchmarkResult
   ```

4. **Validate Threshold**:
   ```
   BenchmarkResult → PerformanceMetrics.threshold_ms → Pass/Fail
   ```

5. **Store Results**:
   ```
   BenchmarkResult → JSON file (tests/fixtures/benchmarks/) + Markdown (PERFORMANCE_BENCHMARKS.md)
   ```

## Storage

### JSON Files

**Location**: `tests/fixtures/benchmarks/`

**Filename Pattern**: `polkadot_mainnet_{block_number}_{algorithm}_{timestamp}.json`

**Example**: `polkadot_mainnet_20000000_sequential-phragmen_2025-01-27T12:00:00Z.json`

**Schema**: Matches `BenchmarkResults` struct serialized to JSON

### Markdown Documentation

**Location**: `PERFORMANCE_BENCHMARKS.md` (repository root)

**Format**: Appended summary table with latest results, hardware specifications, and trend tracking

## Relationships Diagram

```
PolkadotMainnetSnapshot
    │
    ├──> ElectionData (existing)
    │       ├──> candidates: Vec<ValidatorCandidate>
    │       └──> nominators: Vec<Nominator>
    │
    └──> BenchmarkResult
            ├──> PerformanceMetrics
            │       ├──> execution_time_ms
            │       ├──> memory_peak_mb
            │       └──> threshold_ms (algorithm-specific)
            │
            └──> BenchmarkMetadata
                    ├──> hardware_info
                    └──> environment_info
```

## Integration with Existing Models

### Extends Existing Structures

1. **BenchmarkResults** (`tests/common/models.rs`):
   - No changes required - feature uses existing structure
   - Adds new metadata fields via `metadata` HashMap

2. **ElectionData** (`src/models/election_data.rs`):
   - No changes required - feature uses existing structure
   - Fetched via existing `ElectionData::from_rpc()` method

3. **ChainSnapshot** (`tests/common/models.rs`):
   - Similar structure but focused on performance benchmarks
   - May reuse `ChainSnapshot` for caching snapshots

### New Structures

1. **MemoryMeasurementError** (new enum):
   ```rust
   pub enum MemoryMeasurementError {
       UnsupportedPlatform,
       MeasurementFailed(String),
       PlatformError(String),
   }
   ```

2. **MemoryMeasurer** (new trait):
   ```rust
   pub trait MemoryMeasurer {
       fn measure_peak_memory_mb() -> Result<u64, MemoryMeasurementError>;
       fn measure_current_memory_mb() -> Result<u64, MemoryMeasurementError>;
   }
   ```

## Validation Summary

| Entity | Required Fields | Validation Rules |
|--------|----------------|------------------|
| BenchmarkResult | execution_time_ms, memory_peak_mb, metadata | Non-negative values, valid algorithm, positive counts |
| PolkadotMainnetSnapshot | election_data, block_number, rpc_endpoint | Block within 30 days, valid URL, non-empty data |
| PerformanceMetrics | execution_time_ms, memory_peak_mb, algorithm | Execution time <= threshold, algorithm-specific thresholds |
| BenchmarkMetadata | benchmark_name, candidate_count, nominator_count | Non-empty name, positive counts, valid algorithm |

## Notes

- All numeric fields use appropriate Rust types (u64 for time/memory, usize for counts)
- String fields in metadata HashMap allow flexible extension without schema changes
- Timestamps use `chrono::DateTime<Utc>` for consistent timezone handling
- Memory values of 0 indicate measurement unavailable (graceful degradation)
- Block numbers are u64 to support Polkadot's block number range

