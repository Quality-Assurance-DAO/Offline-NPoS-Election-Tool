# Feature Specification: Polkadot Mainnet Performance Benchmarks

**Feature Branch**: `003-polkadot-mainnet-benchmarks`  
**Created**: 2024-12-19  
**Status**: Draft  
**Input**: User description: "improve performance benchmark tests with real Polkadot mainnet data. Refer to Documentation for guidance"

## Clarifications

### Session 2025-01-27

- Q: What performance execution time thresholds should be defined for Polkadot mainnet scale benchmarks? → A: Algorithm-specific thresholds (different targets per algorithm type)
- Q: How should block numbers be selected for benchmarks (fixed, latest, or configurable)? → A: Fixed recent block number (within last 30 days) with optional override parameter
- Q: Which platforms should support memory measurement and what happens on unsupported platforms? → A: Linux/macOS/Windows with graceful degradation (report "N/A" or 0 with warning) on unsupported platforms
- Q: What format and location should be used for documenting benchmark results? → A: JSON files in `tests/fixtures/benchmarks/` plus markdown performance documentation file
- Q: What retry logic and timeout strategy should be used for RPC endpoint failures? → A: Retry up to 3 times with exponential backoff (1s, 2s, 4s), 30s timeout per request, suggest alternative endpoints on failure

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Benchmark Performance with Real Polkadot Mainnet Data (Priority: P1)

A developer needs to verify that the election tool performs efficiently at production scale with real Polkadot mainnet data. They run performance benchmarks using actual on-chain data to measure execution time and ensure the tool can handle real-world scenarios.

**Why this priority**: This is the core gap identified in the documentation. Without real-world benchmarks, developers cannot verify production readiness or predict performance for actual use cases. This delivers immediate value by establishing a baseline for Polkadot mainnet scale.

**Independent Test**: Can be fully tested by fetching real Polkadot mainnet data from a recent block, running an election simulation, and measuring execution time. The test outputs structured benchmark results that can be compared against targets and tracked over time.

**Acceptance Scenarios**:

1. **Given** a Polkadot mainnet RPC endpoint, **When** a developer runs a performance benchmark test, **Then** the system fetches real on-chain election data from a fixed recent block number (within last 30 days) or uses an optionally specified block number, and measures execution time for the election simulation
2. **Given** real Polkadot mainnet data has been fetched, **When** the benchmark test executes, **Then** the system outputs structured benchmark results including execution time, candidate count, nominator count, and block number
3. **Given** benchmark results are generated, **When** a developer reviews the output, **Then** they can verify the tool completes elections within algorithm-specific time thresholds for Polkadot mainnet scale (~300-400 candidates, ~20k-30k nominators): sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s

---

### User Story 2 - Measure Memory Usage During Benchmarks (Priority: P2)

A developer needs to understand memory consumption patterns when running elections with real Polkadot mainnet data to ensure the tool doesn't exceed memory constraints and to identify potential memory leaks.

**Why this priority**: Memory measurement is currently not implemented (returns 0). Understanding memory usage is critical for production deployments and helps identify optimization opportunities. This complements execution time benchmarks to provide a complete performance picture.

**Independent Test**: Can be fully tested by running a benchmark test and verifying that memory usage is measured and reported in the benchmark results. The measurement should capture peak memory usage during election execution.

**Acceptance Scenarios**:

1. **Given** a performance benchmark test is running, **When** the election simulation executes, **Then** the system measures peak memory usage during execution
2. **Given** memory measurement has been collected, **When** benchmark results are generated, **Then** memory usage (in MB) is included in the structured output
3. **Given** multiple benchmark runs are executed, **When** memory usage is tracked over time, **Then** developers can detect memory leaks or excessive memory growth

---

### User Story 3 - Document Performance Characteristics (Priority: P2)

A developer or user needs to understand expected performance characteristics for Polkadot mainnet scale elections to set expectations and make informed decisions about tool usage.

**Why this priority**: Documentation currently lacks actual performance results from real-world data. Documenting performance characteristics helps users understand what to expect and enables performance regression detection. This provides transparency and builds confidence in the tool.

**Independent Test**: Can be fully tested by running benchmarks with real Polkadot mainnet data and verifying that results are documented in a format that can be referenced and updated over time. Documentation should include execution times, memory usage, and hardware specifications.

**Acceptance Scenarios**:

1. **Given** benchmark results have been generated from real Polkadot mainnet data, **When** results are documented, **Then** they are stored as structured JSON files in `tests/fixtures/benchmarks/` directory and appended to a markdown performance documentation file, including execution time, memory usage, candidate count, nominator count, and block number
2. **Given** performance documentation exists, **When** a developer reviews it, **Then** they can understand expected performance for Polkadot mainnet scale elections from both the markdown documentation and structured JSON files
3. **Given** multiple benchmark runs have been executed, **When** results are compared, **Then** developers can track performance trends and detect regressions using both the markdown documentation and programmatic comparison of JSON files

---

### User Story 4 - Integrate Performance Benchmarks into Test Suite (Priority: P3)

A developer needs performance benchmarks to run automatically as part of the test suite to detect performance regressions and ensure consistent performance characteristics over time.

**Why this priority**: While important for long-term maintenance, this can be implemented after the core benchmarking capability exists. Automated regression testing prevents performance degradation but requires the foundational benchmarks first.

**Independent Test**: Can be fully tested by running the test suite and verifying that performance benchmarks execute automatically, output results, and can be configured with performance thresholds that fail builds if exceeded.

**Acceptance Scenarios**:

1. **Given** performance benchmarks are part of the test suite, **When** tests are run, **Then** benchmarks execute automatically and output results
2. **Given** performance thresholds are configured, **When** a benchmark exceeds the threshold, **Then** the test suite fails with a clear error message
3. **Given** benchmark results are generated, **When** tests complete, **Then** results are available for review and comparison with previous runs

---

### Edge Cases

- What happens when the RPC endpoint is unavailable or times out during data fetching? → System retries up to 3 times with exponential backoff (1s, 2s, 4s), with 30s timeout per request; after all retries fail, provides clear error message suggesting alternative RPC endpoints
- How does the system handle blocks with unusually high or low numbers of candidates/nominators?
- What happens when memory measurement fails or is unavailable on the platform? → System reports "N/A" or 0 with a warning message in benchmark results, allowing benchmark to complete successfully
- How does the system handle benchmark failures (election execution errors) vs. performance measurement failures?
- What happens when multiple benchmark tests run concurrently?
- How does the system handle blocks from different eras that may have different data structures?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST fetch real Polkadot mainnet election data (candidates and nominators) from a fixed recent block number (within last 30 days) by default, with optional override to specify a different block number, using RPC endpoints
- **FR-002**: System MUST measure execution time for election simulations using real Polkadot mainnet data
- **FR-003**: System MUST output structured benchmark results including execution time, candidate count, nominator count, block number, and algorithm used
- **FR-004**: System MUST measure peak memory usage during election execution on supported platforms (Linux, macOS, Windows)
- **FR-005**: System MUST include memory usage measurements in benchmark result output, reporting "N/A" or 0 with a warning message when measurement is unavailable on unsupported platforms
- **FR-006**: System MUST support benchmarking with Polkadot mainnet scale data (~300-400 candidates, ~20k-30k nominators)
- **FR-007**: System MUST document benchmark results in structured JSON format stored in `tests/fixtures/benchmarks/` directory and append results to a markdown performance documentation file for human-readable reference
- **FR-008**: System MUST handle RPC endpoint failures gracefully by retrying up to 3 times with exponential backoff (1s, 2s, 4s delays), using a 30s timeout per request, and suggesting alternative endpoints in error messages when all retries fail
- **FR-009**: System MUST support benchmarking multiple algorithms (sequential-phragmen, parallel-phragmen, multi-phase) with the same dataset
- **FR-010**: System MUST allow benchmarks to be run independently or as part of the test suite
- **FR-011**: System MUST validate benchmark execution times against algorithm-specific thresholds: sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s for Polkadot mainnet scale data

### Key Entities *(include if feature involves data)*

- **Benchmark Result**: Represents a single performance benchmark execution, containing execution time, memory usage, candidate count, nominator count, block number (fixed recent default or overridden), algorithm, and metadata
- **Polkadot Mainnet Snapshot**: Represents real on-chain election data fetched from a specific Polkadot mainnet block, containing candidates, nominators, and block metadata
- **Performance Metrics**: Represents measured performance characteristics including execution time (milliseconds) and memory usage (MB)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can run performance benchmarks with real Polkadot mainnet data and receive execution time measurements within 5 minutes of test start
- **SC-002**: Benchmark results include execution time, memory usage, candidate count, nominator count, and block number for at least 95% of successful benchmark runs
- **SC-003**: System successfully benchmarks elections with Polkadot mainnet scale data (~300-400 candidates, ~20k-30k nominators) without errors in at least 90% of attempts
- **SC-004**: Memory usage is measured and reported for at least 90% of benchmark runs on supported platforms (Linux, macOS, Windows); on unsupported platforms, benchmarks complete successfully with "N/A" or 0 reported with a warning message
- **SC-005**: Benchmark results are documented as structured JSON files in `tests/fixtures/benchmarks/` directory and appended to a markdown performance documentation file, enabling both programmatic comparison and human-readable review for performance trend tracking and regression detection
- **SC-006**: System handles RPC endpoint failures gracefully by retrying up to 3 times with exponential backoff and 30s timeout per request, providing clear error messages with alternative endpoint suggestions in 100% of failure scenarios after retries are exhausted
- **SC-007**: Developers can benchmark all three election algorithms (sequential-phragmen, parallel-phragmen, multi-phase) using the same Polkadot mainnet dataset
- **SC-008**: Benchmark execution times meet algorithm-specific thresholds for Polkadot mainnet scale data: sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s in at least 90% of successful runs

## Assumptions

- RPC endpoints for Polkadot mainnet are available and accessible (archive node endpoints recommended for historical blocks)
- Real Polkadot mainnet data contains valid election data (candidates and nominators) at the specified block
- Platform supports memory measurement on Linux, macOS, and Windows; graceful degradation (report "N/A" or 0 with warning) on other platforms
- Developers have network access to fetch data from RPC endpoints
- Benchmark tests may be marked as ignored by default (requiring `--ignored` flag) due to network dependencies
- Execution time and memory usage may vary based on hardware specifications
- Polkadot mainnet scale characteristics (~300-400 candidates, ~20k-30k nominators) are representative of production use cases

## Dependencies

- Existing RPC data fetching capability (`src/input/rpc.rs`)
- Existing benchmark utilities (`tests/common/benchmark_utils.rs`)
- Existing test infrastructure for integration tests
- Access to Polkadot mainnet RPC endpoints (archive nodes recommended)
- Network connectivity for fetching on-chain data

## Out of Scope

- Performance optimization of the election algorithms themselves (this feature measures performance, not optimizes it)
- Benchmarking with synthetic data (already exists, this focuses on real-world data)
- Automated performance regression testing in CI/CD (covered in User Story 4 but may be deferred)
- Benchmarking other Substrate chains beyond Polkadot mainnet (can be added later)
- Real-time performance monitoring during production use
- Performance profiling and bottleneck identification tools
