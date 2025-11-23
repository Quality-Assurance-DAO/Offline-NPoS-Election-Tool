# Offline NPoS Election Tool

A Rust-based offline NPoS (Nominated Proof of Stake) election tool that exactly mirrors the election logic of any Substrate chain. This tool allows you to run election simulations offline, compare different election algorithms, and analyze election outcomes.

## Features

- **Multiple Election Algorithms**: Support for sequential phragmen, parallel phragmen, and multi-phase algorithms
- **Flexible Data Sources**: Fetch data from Substrate RPC endpoints, load from JSON files, or create synthetic data
- **Parameter Overrides**: Modify election parameters (active set size, stakes, voting edges) without changing source data
- **Detailed Diagnostics**: Get explanations for why validators were selected or not selected
- **Multiple Interfaces**: Use via CLI, REST API, or programmatic library API
- **Bit-for-Bit Accuracy**: Produces identical results to on-chain elections using Substrate's native crates

## Installation

### Prerequisites

- Rust 1.70 or later
- Network access (for RPC data fetching, optional for file-based usage)

### Build from Source

```bash
git clone <repository-url>
cd offline-election-tool
cargo build --release
```

The binary will be available at `target/release/offline-election` (or `offline-election.exe` on Windows).

## Quick Start

### Run Election from RPC (On-Chain Data)

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --block-number 12345678
```

### Run Election from JSON File

```bash
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 50 \
  --input-file election_data.json
```

### Get Detailed Diagnostics

```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --diagnostics
```

## Usage

See the [Quickstart Guide](specs/001-offline-npos-election/quickstart.md) for detailed usage examples and documentation.

## Project Structure

```
src/
├── lib.rs                    # Library entry point
├── main.rs                   # CLI binary entry point
├── models/                   # Data models
├── algorithms/               # Election algorithm implementations
├── input/                    # Input data loading (RPC, JSON, synthetic)
├── diagnostics/              # Diagnostic generation
├── cli/                      # CLI interface
└── api/                      # REST API server

tests/
├── unit/                     # Unit tests
├── integration/              # Integration tests
└── contract/                 # Contract tests
```

## Documentation

- [Feature Specification](specs/001-offline-npos-election/spec.md)
- [Implementation Plan](specs/001-offline-npos-election/plan.md)
- [Data Model](specs/001-offline-npos-election/data-model.md)
- [Quickstart Guide](specs/001-offline-npos-election/quickstart.md)
- [Programmatic API](specs/001-offline-npos-election/contracts/programmatic-api.md)
- [REST API](specs/001-offline-npos-election/contracts/rest-api.yaml)

## License

MIT OR Apache-2.0
