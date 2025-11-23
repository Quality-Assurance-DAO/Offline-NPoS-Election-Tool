# Link Inventory: Internal Documentation Links

**Created**: 2025-01-27  
**Purpose**: Complete inventory of all internal links in documentation files for systematic update

## Link Summary

**Total Internal Links Found**: 57+ links across documentation files

## Links by Source File

### README.md

| Link Text | Target | Type | Status | New Target |
|-----------|--------|------|--------|------------|
| PERFORMANCE_BENCHMARKS.md | `PERFORMANCE_BENCHMARKS.md` | Internal | Needs Update | `docs/guides/performance.md` |
| RPC_ARCHIVE_NODES.md | `RPC_ARCHIVE_NODES.md` | Internal | Needs Update | `docs/guides/rpc-usage.md` |
| API_USAGE.md | `API_USAGE.md` | Internal | Needs Update | `docs/api/rest-api.md` or `docs/api/programmatic-api.md` |
| TESTING.md | `TESTING.md` | Internal | Needs Update | `docs/testing/overview.md` |
| ALGORITHM_EXTENSIBILITY.md | `ALGORITHM_EXTENSIBILITY.md` | Internal | Needs Update | `docs/guides/algorithms.md` |
| Quickstart Guide | `specs/001-offline-npos-election/quickstart.md` | Internal | Keep | (unchanged) |
| Feature Specification | `specs/001-offline-npos-election/spec.md` | Internal | Keep | (unchanged) |
| Implementation Plan | `specs/001-offline-npos-election/plan.md` | Internal | Keep | (unchanged) |
| Data Model | `specs/001-offline-npos-election/data-model.md` | Internal | Keep | (unchanged) |
| Programmatic API | `specs/001-offline-npos-election/contracts/programmatic-api.md` | Internal | Keep | (unchanged) |
| REST API | `specs/001-offline-npos-election/contracts/rest-api.yaml` | Internal | Keep | (unchanged) |

### API_USAGE.md

| Link Text | Target | Type | Status | New Target |
|-----------|--------|------|--------|------------|
| Security and Robustness | `../README.md#security-and-robustness` | Internal | Keep | `../README.md#security-and-robustness` |

### RPC_TESTING.md

| Link Text | Target | Type | Status | New Target |
|-----------|--------|------|--------|------------|
| RPC_ARCHIVE_NODES.md | `RPC_ARCHIVE_NODES.md` | Internal | Needs Update | `docs/guides/rpc-usage.md` |

### tests/README.md

| Link Text | Target | Type | Status | New Target |
|-----------|--------|------|--------|------------|
| PERFORMANCE_BENCHMARKS.md | `../PERFORMANCE_BENCHMARKS.md` | Internal | Needs Update | `../docs/guides/performance.md` |

## Link Mapping Table

| Old Link Target | New Link Target | Notes |
|----------------|-----------------|-------|
| `API_USAGE.md` | `docs/api/rest-api.md` or `docs/api/programmatic-api.md` | Split into two files - need to determine which link goes where |
| `ALGORITHM_EXTENSIBILITY.md` | `docs/guides/algorithms.md` | Renamed |
| `PERFORMANCE_BENCHMARKS.md` | `docs/guides/performance.md` | Renamed |
| `RPC_ARCHIVE_NODES.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `RPC_TESTING.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `TESTING.md` | `docs/testing/overview.md` | Consolidated |
| `TEST_RUNNING_INSTRUCTIONS.md` | `docs/testing/overview.md` | Consolidated |
| `RFP_COMPLIANCE_ASSESSMENT.md` | `docs/reference/rfp-compliance.md` | Renamed |

## Link Types

### Internal File Links
- Links between root-level documentation files
- Links from root-level files to specs/ files
- Links from tests/README.md to root-level files

### Section Links (Same File)
- Links to sections within same file (e.g., `#security-and-robustness`)
- These will be preserved if sections remain

### External Links
- Links to external resources (Polkadot docs, GitHub, etc.)
- These remain unchanged

### Code References
- References to code files (e.g., `src/algorithms/sequential_phragmen.rs`)
- These remain unchanged

## Update Priority

1. **High Priority**: Links in README.md (main entry point)
2. **Medium Priority**: Links in files being consolidated
3. **Low Priority**: Links in files being moved (can update after move)

## Notes

- All internal links must be updated before removing old files
- Section anchors may need verification after consolidation
- Links to specs/ files remain unchanged (specs/ directory not affected)
- External links remain unchanged

