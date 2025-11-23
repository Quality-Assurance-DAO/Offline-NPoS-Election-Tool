# Performance Benchmark Output Contract

**Date**: 2025-01-27  
**Feature**: 002-comprehensive-testing

## Overview

This document defines the contract for performance benchmark output in structured JSON format. Benchmarks measure election execution time, memory usage, and other performance metrics.

## Output Format

Performance benchmarks output structured JSON with timing, memory usage, and metadata for automated comparison and CI integration.

## Schema

### Benchmark Output Schema

```json
{
  "timing_ms": "u64 (required, execution time in milliseconds)",
  "memory_mb": "u64 (required, peak memory usage in MB)",
  "metadata": {
    "benchmark_name": "string (required)",
    "candidate_count": "u32 (required)",
    "nominator_count": "u32 (required)",
    "algorithm": "sequential-phragmen | parallel-phragmen | multi-phase (required)",
    "iterations": "u32 (optional, number of iterations run)",
    "mean_time_ms": "f64 (optional, mean execution time if iterations > 1)",
    "std_dev_ms": "f64 (optional, standard deviation if iterations > 1)",
    "min_time_ms": "u64 (optional, minimum execution time)",
    "max_time_ms": "u64 (optional, maximum execution time)",
    "hardware_info": {
      "cpu_cores": "u32 (optional)",
      "ram_gb": "u32 (optional)",
      "os": "string (optional)"
    } (optional),
    "rust_version": "string (optional)",
    "timestamp": "ISO 8601 datetime (optional)"
  }
}
```

## Validation Rules

1. **Timing**:
   - `timing_ms` must be non-negative
   - If `iterations > 1`, `mean_time_ms` and `std_dev_ms` must be present
   - `min_time_ms` <= `mean_time_ms` <= `max_time_ms` (if all present)

2. **Memory**:
   - `memory_mb` must be non-negative
   - Represents peak memory usage during benchmark execution

3. **Metadata**:
   - `benchmark_name` must be non-empty and unique
   - `candidate_count` and `nominator_count` must be positive
   - `algorithm` must be one of the supported algorithms

## Example Output

### Single Iteration Benchmark

```json
{
  "timing_ms": 1234,
  "memory_mb": 256,
  "metadata": {
    "benchmark_name": "large_scale_1k_candidates",
    "candidate_count": 1000,
    "nominator_count": 10000,
    "algorithm": "sequential-phragmen",
    "iterations": 1,
    "timestamp": "2025-01-27T10:00:00Z"
  }
}
```

### Multi-Iteration Benchmark (with Statistical Analysis)

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
    "std_dev_ms": 45.2,
    "min_time_ms": 1189,
    "max_time_ms": 1320,
    "hardware_info": {
      "cpu_cores": 8,
      "ram_gb": 16,
      "os": "macOS 14.0"
    },
    "rust_version": "1.70.0",
    "timestamp": "2025-01-27T10:00:00Z"
  }
}
```

## Performance Targets

Benchmarks should validate against these performance targets:

1. **1,000 candidates / 10,000 nominators**: < 60,000 ms (60 seconds)
2. **5,000 candidates / 50,000 nominators**: < 300,000 ms (5 minutes)
3. **10,000 candidates / 100,000 nominators**: No OOM on 8GB+ RAM systems

## Output Location

Benchmark results are written to:
- Standard output (stdout) for CI integration
- Optional file: `tests/fixtures/benchmarks/{benchmark_name}_{timestamp}.json`

## Comparison and Regression Detection

Benchmark output can be compared across runs to detect performance regressions:
- Compare `mean_time_ms` across versions
- Detect if execution time increases beyond acceptable threshold (e.g., 10% increase)
- Compare `memory_mb` to detect memory usage increases
- Use `std_dev_ms` to account for measurement variance

## Error Handling

If benchmark execution fails:
- Output error JSON:
  ```json
  {
    "error": "string (error message)",
    "benchmark_name": "string",
    "timestamp": "ISO 8601 datetime"
  }
  ```
- Exit with non-zero status code
- Include error details in error message

