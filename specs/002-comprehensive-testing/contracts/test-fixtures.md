# Test Fixtures Contract

**Date**: 2025-01-27  
**Feature**: 002-comprehensive-testing

## Overview

This document defines the contract for test fixture JSON files used in the comprehensive test suite. Test fixtures represent election datasets with input data, configuration, and expected results.

## File Location

Test fixtures are stored in version-controlled JSON files:
- Edge case fixtures: `tests/fixtures/regression/edge_cases/{test_name}.json`
- Regression fixtures: `tests/fixtures/regression/{test_name}.json`
- Chain snapshots: `tests/fixtures/chain_snapshots/{chain}/{block_number}.json`

## Schema

### Test Fixture Schema

```json
{
  "metadata": {
    "test_name": "string (required, unique)",
    "description": "string (required)",
    "created": "ISO 8601 datetime (required)",
    "algorithm": "sequential-phragmen | parallel-phragmen | multi-phase (required)",
    "category": "edge_case | performance | regression | chain_snapshot (required)",
    "tags": ["string"] (optional)
  },
  "input": {
    "candidates": [
      {
        "account_id": "string (SS58, required)",
        "stake": "u128 (required)",
        "metadata": {} (optional)
      }
    ],
    "nominators": [
      {
        "account_id": "string (SS58, required)",
        "stake": "u128 (required)",
        "targets": ["string (SS58 account IDs, required)"],
        "metadata": {} (optional)
      }
    ],
    "config": {
      "active_set_size": "u32 (required)",
      "algorithm": "sequential-phragmen | parallel-phragmen | multi-phase (required)",
      "overrides": {} (optional)
    }
  },
  "expected_result": {
    "selected_validators": [
      {
        "account_id": "string (SS58, required)",
        "total_backing": "u128 (required)",
        "nominator_count": "u32 (required)"
      }
    ],
    "stake_allocations": {
      "nominator_account_id": {
        "validator_account_id": "u128 (stake amount)"
      }
    }
  } (optional, required for regression tests)
}
```

## Validation Rules

1. **Metadata**:
   - `test_name` must be unique within the category
   - `description` must be non-empty
   - `created` must be valid ISO 8601 datetime
   - `algorithm` must be one of the supported algorithms
   - `category` must be one of the defined categories

2. **Input**:
   - Must contain at least one candidate (unless testing zero candidates edge case)
   - Must contain at least one nominator (unless testing zero nominators edge case)
   - All candidate account IDs must be unique
   - All nominator account IDs must be unique
   - All voting targets must reference existing candidate account IDs
   - `active_set_size` must be positive and <= candidate count

3. **Expected Result** (if provided):
   - `selected_validators` must contain exactly `active_set_size` validators
   - All selected validator account IDs must exist in input candidates
   - All stake allocations must reference valid nominator and validator account IDs
   - Total stake allocations for a validator must match `total_backing`

## Example: Edge Case Fixture

```json
{
  "metadata": {
    "test_name": "zero_candidates",
    "description": "Edge case: election with zero validator candidates",
    "created": "2025-01-27T10:00:00Z",
    "algorithm": "sequential-phragmen",
    "category": "edge_case",
    "tags": ["zero_candidates", "error_case"]
  },
  "input": {
    "candidates": [],
    "nominators": [
      {
        "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "stake": 1000000000,
        "targets": []
      }
    ],
    "config": {
      "active_set_size": 3,
      "algorithm": "sequential-phragmen"
    }
  },
  "expected_result": null
}
```

## Example: Regression Test Fixture

```json
{
  "metadata": {
    "test_name": "normal_election_5x5",
    "description": "Normal election with 5 candidates and 5 nominators",
    "created": "2025-01-27T10:00:00Z",
    "algorithm": "sequential-phragmen",
    "category": "regression",
    "tags": ["normal_case", "baseline"]
  },
  "input": {
    "candidates": [
      {
        "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "stake": 1000000000
      }
    ],
    "nominators": [
      {
        "account_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "stake": 500000000,
        "targets": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]
      }
    ],
    "config": {
      "active_set_size": 3,
      "algorithm": "sequential-phragmen"
    }
  },
  "expected_result": {
    "selected_validators": [
      {
        "account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "total_backing": 1500000000,
        "nominator_count": 1
      }
    ],
    "stake_allocations": {
      "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty": {
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY": 500000000
      }
    }
  }
}
```

## Chain Snapshot Schema

```json
{
  "metadata": {
    "chain": "string (required, e.g., 'polkadot', 'kusama')",
    "block_number": "u64 (required)",
    "timestamp": "ISO 8601 datetime (required)",
    "rpc_endpoint": "string (URL, required)",
    "expected_validators": ["string (SS58 account IDs, required)"],
    "expected_stake_allocations": {
      "nominator_account_id": {
        "validator_account_id": "u128 (stake amount)"
      }
    } (required)
  },
  "election_data": {
    "candidates": [...],
    "nominators": [...]
  }
}
```

## Error Handling

When loading test fixtures:
- **Invalid JSON**: Return error with file path and JSON parsing error
- **Missing required fields**: Return error listing missing fields
- **Invalid account IDs**: Return error with invalid account ID and position
- **Invalid references**: Return error when voting targets reference non-existent candidates
- **Schema validation failures**: Return error with specific validation failure details

## Versioning

Test fixtures are version-controlled JSON files. When updating fixtures:
- Update `created` timestamp if modifying input data
- Update `expected_result` only when intentional algorithm changes occur
- Document changes in commit messages
- Preserve historical fixtures for regression testing

