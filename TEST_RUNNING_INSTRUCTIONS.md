# Test Running Instructions

This document provides comprehensive instructions for running all tests in the Offline Election Tool project.

## Prerequisites

- Rust 1.70 or later installed
- Cargo (comes with Rust)
- Network access (for integration tests that use RPC endpoints)

## Quick Start

### Run All Tests

```bash
cargo test
```

This will run all unit tests and integration tests.

## Test Types

The project has several types of tests:

1. **Unit Tests** - Tests in `src/` files (marked with `#[cfg(test)]`)
2. **Integration Tests** - Tests in `tests/` directory
   - Edge case tests (`tests/integration/edge_cases/`)
   - Chain snapshot tests (`tests/integration/chain_snapshots/`)
   - Performance tests (`tests/integration/performance/`)
   - Regression tests (`tests/integration/regression/`)

## Running Specific Test Categories

### Run Only Unit Tests

```bash
cargo test --lib
```

### Run Only Integration Tests

```bash
cargo test --test '*'
```

Or run a specific integration test file:

```bash
cargo test --test integration_edge_cases_zero_candidates
```

### Run Edge Case Tests

```bash
# Run all edge case tests
cargo test --test integration_edge_cases_zero_candidates

# Run specific edge case test
cargo test test_zero_candidates_should_fail
```

### Run Tests by Name Pattern

```bash
# Run all tests containing "zero" in their name
cargo test zero

# Run all tests containing "candidate" in their name
cargo test candidate
```

## Running Tests with Output

### Show Test Output (for passing tests)

```bash
cargo test -- --nocapture
```

### Show Test Output (for all tests)

```bash
cargo test -- --show-output
```

### Run Tests in Single Thread (useful for debugging)

```bash
cargo test -- --test-threads=1
```

## Running Specific Tests

### Run a Single Test by Name

```bash
cargo test test_zero_candidates_should_fail
```

### Run Tests Matching a Pattern

```bash
# Run all tests starting with "test_zero"
cargo test test_zero

# Run all tests in a specific module
cargo test integration::edge_cases
```

## Running Tests with Filtering

### Skip Tests That Require Network Access

If you want to skip tests that require RPC access:

```bash
# Run tests but skip those marked with #[ignore]
cargo test -- --skip rpc
```

### Run Only Ignored Tests

```bash
cargo test -- --ignored
```

## Running Tests in Different Modes

### Run Tests in Release Mode

```bash
cargo test --release
```

This compiles with optimizations and may catch different issues.

### Run Tests with Verbose Output

```bash
cargo test --verbose
```

## Integration Test Details

### Edge Case Tests

The edge case tests verify handling of boundary conditions:

- `test_zero_candidates.rs` - Tests with no candidates
- `test_zero_nominators.rs` - Tests with no nominators
- `test_single_candidate.rs` - Tests with single candidate
- `test_single_nominator.rs` - Tests with single nominator
- `test_zero_candidate_stakes.rs` - Tests with zero-stake candidates
- `test_max_active_set_size.rs` - Tests with maximum active set size
- `test_empty_voting_edges.rs` - Tests with no voting edges
- `test_duplicate_account_ids.rs` - Tests with duplicate account IDs

Run all edge case tests:

```bash
cargo test --test integration_edge_cases_zero_candidates
```

### Chain Snapshot Tests

These tests use real on-chain data:

```bash
# Run chain snapshot tests (requires network access)
cargo test --test chain_snapshots
```

**Note**: These tests may require network access and could be slow.

### Performance Tests

Run performance benchmarks:

```bash
cargo test --test performance
```

Or use Criterion benchmarks:

```bash
cargo bench
```

## Test Output Examples

### Successful Test Run

```
running 15 tests
test test_zero_candidates_should_fail ... ok
test test_single_candidate ... ok
test test_zero_nominators ... ok
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Failed Test Run

```
running 1 test
test test_zero_candidates_should_fail ... FAILED

failures:

---- test_zero_candidates_should_fail stdout ----
thread 'test_zero_candidates_should_fail' panicked at 'assertion failed: ...'
```

## Debugging Tests

### Run Tests with Backtrace

```bash
RUST_BACKTRACE=1 cargo test
```

Or on Windows:

```cmd
set RUST_BACKTRACE=1
cargo test
```

### Run Tests with Full Backtrace

```bash
RUST_BACKTRACE=full cargo test
```

### Run a Single Test with Debug Output

```bash
RUST_LOG=debug cargo test test_zero_candidates_should_fail -- --nocapture
```

## Continuous Testing

### Watch for Changes and Re-run Tests

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

Then run:

```bash
cargo watch -x test
```

This will automatically re-run tests when files change.

## Test Coverage

### Generate Test Coverage Report

Install `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

Then run:

```bash
cargo tarpaulin --out Html
```

This generates an HTML coverage report in `tarpaulin-report.html`.

## Common Test Commands Reference

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests matching pattern
cargo test pattern

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run tests in release mode
cargo test --release

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests in single thread
cargo test -- --test-threads=1

# Run ignored tests
cargo test -- --ignored

# Skip tests matching pattern
cargo test -- --skip pattern
```

## Troubleshooting

### Tests Fail Due to Network Issues

If tests that require RPC access are failing:

1. Check your internet connection
2. Verify the RPC endpoint is accessible
3. Consider skipping network-dependent tests: `cargo test -- --skip rpc`

### Tests Fail Due to Timeout

Some RPC tests may timeout. Increase timeout or skip them:

```bash
# Skip slow tests
cargo test -- --skip slow
```

### Tests Fail Due to Missing Fixtures

Some tests require fixture files. Ensure test fixtures are present:

```bash
# Check if fixtures exist
ls tests/fixtures/
```

### Compilation Errors in Tests

If tests fail to compile:

```bash
# Clean and rebuild
cargo clean
cargo test
```

## CI/CD Integration

For continuous integration, use:

```bash
# Run all tests
cargo test --all-features

# Run tests with output
cargo test -- --nocapture --test-threads=1

# Run tests and fail on warnings
RUSTFLAGS="-D warnings" cargo test
```

## Next Steps

- See `TESTING.md` for more details on test data and scenarios
- See `README.md` for general usage instructions
- Check test source files in `tests/` for test implementation details

