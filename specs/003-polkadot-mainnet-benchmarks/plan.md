# Implementation Plan: Polkadot Mainnet Performance Benchmarks

**Branch**: `003-polkadot-mainnet-benchmarks` | **Date**: 2025-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-polkadot-mainnet-benchmarks/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Enhance the existing performance benchmark infrastructure to support real-world Polkadot mainnet data benchmarks. The feature adds execution time and memory usage measurement for elections using actual on-chain data from recent Polkadot mainnet blocks (~300-400 candidates, ~20k-30k nominators). Benchmarks will fetch data via RPC with retry logic, measure performance metrics, validate against algorithm-specific thresholds, and document results in structured JSON files and markdown documentation. This addresses the gap where existing benchmarks only use synthetic data and don't measure real-world performance characteristics.

## Technical Context

**Language/Version**: Rust 1.70+ (MSRV - Minimum Supported Rust Version)  
**Primary Dependencies**: 
- Existing: `sp-npos-elections`, `frame-election-provider-support`, `pallet-election-provider-multi-phase`, `jsonrpsee` (RPC client), `tokio` (async runtime)
- Testing: `cargo test` with integration tests, existing `tests/common/benchmark_utils.rs` utilities
- Memory measurement: Platform-specific APIs (`libc` for macOS, `winapi` for Windows, `/proc/self/status` for Linux) with trait-based abstraction for cross-platform support

**Storage**: File-based JSON storage in `tests/fixtures/benchmarks/` directory for benchmark results  
**Testing**: `cargo test` with integration tests, existing benchmark utilities in `tests/common/benchmark_utils.rs`  
**Target Platform**: Cross-platform (Linux, macOS, Windows) - Rust native compilation  
**Project Type**: Single Rust project (library + CLI + optional REST server)  
**Performance Goals**: 
- Algorithm-specific execution time thresholds: sequential-phragmen < 30s, parallel-phragmen < 15s, multi-phase < 45s for Polkadot mainnet scale (~300-400 candidates, ~20k-30k nominators)
- Memory measurement on supported platforms (Linux, macOS, Windows) with graceful degradation on unsupported platforms

**Constraints**: 
- RPC endpoint availability and network connectivity required
- 30s timeout per RPC request with exponential backoff retry (1s, 2s, 4s delays, up to 3 retries)
- Fixed recent block number (within last 30 days) by default, with optional override parameter
- Memory measurement must gracefully degrade on unsupported platforms (report "N/A" or 0 with warning)

**Scale/Scope**: 
- Polkadot mainnet scale: ~300-400 candidates, ~20k-30k nominators
- Support benchmarking all three algorithms (sequential-phragmen, parallel-phragmen, multi-phase) with same dataset
- Benchmark results stored as JSON files in `tests/fixtures/benchmarks/` plus markdown performance documentation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Note**: Constitution file (`.specify/memory/constitution.md`) appears to be a template and hasn't been customized for this project. Proceeding with standard development practices.

### Pre-Phase 0 Gates (Initial Check)

**Gates**:
- ✅ **Test-First**: Feature includes comprehensive test scenarios and acceptance criteria
- ✅ **Integration Testing**: Feature adds integration tests for performance benchmarks
- ✅ **Documentation**: Feature requires documentation of benchmark results and performance characteristics
- ✅ **Error Handling**: Feature includes retry logic and graceful error handling for RPC failures
- ✅ **Cross-Platform**: Feature supports Linux, macOS, Windows with graceful degradation for unsupported platforms

**No violations identified** - Feature aligns with standard development practices and extends existing test infrastructure.

### Post-Phase 1 Design Check

**Re-evaluation after design completion**:

- ✅ **Data Model**: Well-defined entities (BenchmarkResult, PolkadotMainnetSnapshot, PerformanceMetrics) with clear validation rules
- ✅ **API Contracts**: Structured JSON output format with Polkadot-specific fields and threshold validation
- ✅ **Error Handling**: Comprehensive error handling for RPC failures, memory measurement failures, and threshold violations
- ✅ **Documentation**: Quickstart guide, data model documentation, and contract specifications complete
- ✅ **Integration**: Design integrates cleanly with existing benchmark infrastructure and test utilities

**No violations identified** - Design maintains consistency with existing codebase patterns and extends functionality appropriately.

## Project Structure

### Documentation (this feature)

```text
specs/003-polkadot-mainnet-benchmarks/
├── plan.md                                    # This file (/speckit.plan command output)
├── spec.md                                    # Feature specification (input)
├── research.md                                # Phase 0 output ✅ COMPLETE
├── data-model.md                              # Phase 1 output ✅ COMPLETE
├── quickstart.md                              # Phase 1 output ✅ COMPLETE
├── contracts/
│   └── polkadot-mainnet-benchmark-output.md  # Phase 1 output ✅ COMPLETE
├── checklists/
│   └── requirements.md                        # Requirements checklist (existing)
└── tasks.md                                   # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── algorithms/          # Election algorithm implementations
├── api/                # REST API handlers
├── cli/                # CLI command handlers
├── diagnostics/        # Election result diagnostics
├── engine.rs           # Election engine orchestration
├── error.rs            # Error types
├── input/              # Data input loaders (RPC, JSON, synthetic)
│   ├── rpc.rs         # RPC data fetching (existing, will enhance)
│   ├── json.rs
│   └── synthetic.rs
├── models/             # Data models (ElectionData, ElectionResult, etc.)
├── types.rs            # Type definitions
├── lib.rs              # Library entry point
└── main.rs             # CLI binary entry point

tests/
├── common/             # Shared test utilities
│   ├── benchmark_utils.rs    # Benchmark utilities (existing, will enhance)
│   ├── rpc_utils.rs          # RPC utilities (existing)
│   ├── rpc_retry.rs          # RPC retry logic (existing)
│   └── models.rs             # Test data models (existing)
├── fixtures/
│   └── benchmarks/           # Benchmark result JSON files (to be created)
├── integration/
│   └── performance/          # Performance integration tests (existing, will enhance)
└── unit/                     # Unit tests

benches/
└── large_scale_benchmark.rs   # Criterion benchmarks (existing)
```

**Structure Decision**: Single Rust project structure. Feature extends existing test infrastructure:
- Enhances `tests/common/benchmark_utils.rs` with memory measurement
- Adds new integration tests in `tests/integration/performance/` for Polkadot mainnet benchmarks
- Creates `tests/fixtures/benchmarks/` directory for structured JSON benchmark results
- Extends existing RPC utilities with retry logic (already exists in `tests/common/rpc_retry.rs`)

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (None) | N/A | N/A |

## Phase Completion Status

### Phase 0: Outline & Research ✅ COMPLETE

**Output**: `research.md`

**Research Completed**:
- ✅ Memory measurement implementation strategy (platform-specific APIs with trait abstraction)
- ✅ RPC retry logic and error handling (use existing `retry_with_backoff` utilities)
- ✅ Benchmark result storage format (JSON files + markdown documentation)
- ✅ Block number selection strategy (fixed recent block with optional override)
- ✅ Performance threshold validation (algorithm-specific thresholds with test validation)
- ✅ Benchmark test integration (`#[ignore]` attribute for network-dependent tests)

**All NEEDS CLARIFICATION items resolved**.

### Phase 1: Design & Contracts ✅ COMPLETE

**Outputs**:
- ✅ `data-model.md` - Complete data model with entities, relationships, and validation rules
- ✅ `contracts/polkadot-mainnet-benchmark-output.md` - API contract for benchmark output format
- ✅ `quickstart.md` - Quickstart guide for running benchmarks
- ✅ Agent context updated via `.specify/scripts/bash/update-agent-context.sh cursor-agent`

**Design Artifacts**:
- **Data Model**: BenchmarkResult, PolkadotMainnetSnapshot, PerformanceMetrics, BenchmarkMetadata entities defined
- **Contracts**: JSON output schema with Polkadot-specific fields, validation rules, and error handling
- **Quickstart**: Complete guide with examples, configuration, troubleshooting, and best practices

**Agent Context**: Updated Cursor IDE context file with Rust 1.70+ and file-based JSON storage information.

### Phase 2: Task Breakdown

**Status**: Pending - To be completed by `/speckit.tasks` command

**Note**: Phase 2 planning (task breakdown) is handled by a separate command (`/speckit.tasks`). This plan stops after Phase 1 design completion as specified in the workflow.

## Summary

**Branch**: `003-polkadot-mainnet-benchmarks`  
**IMPL_PLAN Path**: `/Users/stephen/Documents/GitHub/Offline-election-tool/specs/003-polkadot-mainnet-benchmarks/plan.md`  
**Generated Artifacts**:
- `research.md` - Research findings and technical decisions
- `data-model.md` - Complete data model documentation
- `contracts/polkadot-mainnet-benchmark-output.md` - API contract specification
- `quickstart.md` - Quickstart guide for developers

**Next Steps**: Run `/speckit.tasks` command to generate task breakdown for implementation.
