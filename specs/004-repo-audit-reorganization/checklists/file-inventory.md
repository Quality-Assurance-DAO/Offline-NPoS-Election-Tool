# File Inventory: Root-Level Markdown Files

**Created**: 2025-01-27  
**Purpose**: Complete inventory of all root-level markdown files for audit and reorganization

## Root-Level Markdown Files

| File Name | Size (lines) | Purpose | Target Location | Status |
|-----------|--------------|---------|-----------------|--------|
| `README.md` | ~600 | Main entry point | Root (enhanced) | Keep & Enhance |
| `ALGORITHM_EXTENSIBILITY.md` | ~434 | Algorithm extensibility guide | `docs/guides/algorithms.md` | Move & Rename |
| `API_USAGE.md` | ~902 | Comprehensive API documentation | `docs/api/rest-api.md` + `docs/api/programmatic-api.md` | Split & Move |
| `PERFORMANCE_BENCHMARKS.md` | Unknown | Performance benchmarks guide | `docs/guides/performance.md` | Move & Rename |
| `RPC_ARCHIVE_NODES.md` | Unknown | RPC archive node guide | `docs/guides/rpc-usage.md` | Consolidate & Move |
| `RPC_TESTING.md` | Unknown | RPC testing guide | `docs/guides/rpc-usage.md` | Consolidate & Move |
| `TESTING.md` | ~177 | Testing overview | `docs/testing/overview.md` | Consolidate & Move |
| `TEST_RUNNING_INSTRUCTIONS.md` | ~373 | Test running instructions | `docs/testing/overview.md` | Consolidate & Move |
| `RFP_COMPLIANCE_ASSESSMENT.md` | Unknown | RFP compliance assessment | `docs/reference/rfp-compliance.md` | Move & Rename |

## File Categories

### Entry Point Files
- **README.md**: Main entry point (stays at root, needs enhancement)

### Guide Files (to move to `docs/guides/`)
- **ALGORITHM_EXTENSIBILITY.md**: Algorithm extensibility guide
- **PERFORMANCE_BENCHMARKS.md**: Performance benchmarks guide
- **RPC_ARCHIVE_NODES.md**: RPC archive node guide
- **RPC_TESTING.md**: RPC testing guide

### API Documentation (to move to `docs/api/`)
- **API_USAGE.md**: Comprehensive API documentation (needs split)

### Testing Documentation (to move to `docs/testing/`)
- **TESTING.md**: Testing overview
- **TEST_RUNNING_INSTRUCTIONS.md**: Test running instructions

### Reference Documentation (to move to `docs/reference/`)
- **RFP_COMPLIANCE_ASSESSMENT.md**: RFP compliance assessment

## Consolidation Candidates

### High Overlap (>60% overlap)
- **TESTING.md** + **TEST_RUNNING_INSTRUCTIONS.md**: Both cover test execution
- **RPC_ARCHIVE_NODES.md** + **RPC_TESTING.md**: Both cover RPC usage

### Split Candidates
- **API_USAGE.md**: Large file covering both REST API and programmatic API

## Notes

- All files are markdown format (.md)
- Files are located at repository root level
- No duplicates identified yet (requires similarity analysis)
- File sizes estimated from initial reads

