# File Mapping Table: Old Paths → New Paths

**Created**: 2025-01-27  
**Purpose**: Complete mapping of old file paths to new paths per contracts/documentation-structure.md

## Mapping Table

| Old Path | New Path | Action | Status |
|----------|----------|--------|--------|
| `README.md` | `README.md` | Enhance (add content, update links) | Pending |
| `ALGORITHM_EXTENSIBILITY.md` | `docs/guides/algorithms.md` | Move & Rename | Pending |
| `API_USAGE.md` | `docs/api/rest-api.md` | Split & Move (REST API portion) | Pending |
| `API_USAGE.md` | `docs/api/programmatic-api.md` | Split & Move (Programmatic API portion) | Pending |
| `PERFORMANCE_BENCHMARKS.md` | `docs/guides/performance.md` | Move & Rename | Pending |
| `RPC_ARCHIVE_NODES.md` | `docs/guides/rpc-usage.md` | Consolidate & Move | Pending |
| `RPC_TESTING.md` | `docs/guides/rpc-usage.md` | Consolidate & Move | Pending |
| `TESTING.md` | `docs/testing/overview.md` | Consolidate & Move | Pending |
| `TEST_RUNNING_INSTRUCTIONS.md` | `docs/testing/overview.md` | Consolidate & Move | Pending |
| `RFP_COMPLIANCE_ASSESSMENT.md` | `docs/reference/rfp-compliance.md` | Move & Rename | Pending |

## Files to Create

| New Path | Purpose | Source |
|----------|---------|--------|
| `docs/polkadot/ecosystem-overview.md` | Polkadot ecosystem context | New content (required by FR-007, FR-008) |
| `docs/reference/glossary.md` | Technical terms glossary | Consolidated from all docs |

## Files to Update (Not Moved)

| File Path | Changes |
|-----------|---------|
| `README.md` | Add Polkadot overview summary, update navigation links, add quick start, add glossary links |
| `tests/README.md` | Update links to point to `docs/testing/overview.md` |

## Files to Remove (After Consolidation)

After consolidation and link updates are complete, remove:
- `ALGORITHM_EXTENSIBILITY.md`
- `API_USAGE.md`
- `PERFORMANCE_BENCHMARKS.md`
- `RPC_ARCHIVE_NODES.md`
- `RPC_TESTING.md`
- `TESTING.md`
- `TEST_RUNNING_INSTRUCTIONS.md`
- `RFP_COMPLIANCE_ASSESSMENT.md`

## Consolidation Details

### Consolidation 1: Testing Documentation
**Source Files**:
- `TESTING.md`
- `TEST_RUNNING_INSTRUCTIONS.md`

**Target File**: `docs/testing/overview.md`

**Content Strategy**:
- Merge overlapping content
- Preserve all unique sections
- Organize by topic (Quick Start, Test Data, Running Tests, Troubleshooting)

### Consolidation 2: RPC Documentation
**Source Files**:
- `RPC_ARCHIVE_NODES.md`
- `RPC_TESTING.md`

**Target File**: `docs/guides/rpc-usage.md`

**Content Strategy**:
- Merge overlapping content
- Preserve all unique sections
- Organize by topic (Overview, Quick Start, Archive Nodes, Troubleshooting)

### Split: API Documentation
**Source File**: `API_USAGE.md`

**Target Files**:
- `docs/api/rest-api.md` (REST API server documentation)
- `docs/api/programmatic-api.md` (Programmatic library API)

**Split Strategy**:
- Identify REST API sections vs. programmatic API sections
- Create two separate files
- Preserve all content

## Link Update Mapping

| Old Link Target | New Link Target | Notes |
|----------------|-----------------|-------|
| `API_USAGE.md` | `docs/api/rest-api.md` or `docs/api/programmatic-api.md` | Split - determine which link goes where |
| `ALGORITHM_EXTENSIBILITY.md` | `docs/guides/algorithms.md` | Direct mapping |
| `PERFORMANCE_BENCHMARKS.md` | `docs/guides/performance.md` | Direct mapping |
| `RPC_ARCHIVE_NODES.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `RPC_TESTING.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `TESTING.md` | `docs/testing/overview.md` | Consolidated |
| `TEST_RUNNING_INSTRUCTIONS.md` | `docs/testing/overview.md` | Consolidated |
| `RFP_COMPLIANCE_ASSESSMENT.md` | `docs/reference/rfp-compliance.md` | Direct mapping |

## Execution Order

1. **Create new directory structure** ✅ (Phase 1 complete)
2. **Create new files** (ecosystem-overview.md, glossary.md)
3. **Move unique files** (ALGORITHM_EXTENSIBILITY.md, PERFORMANCE_BENCHMARKS.md, RFP_COMPLIANCE_ASSESSMENT.md)
4. **Split API_USAGE.md** (create rest-api.md and programmatic-api.md)
5. **Consolidate testing docs** (merge TESTING.md + TEST_RUNNING_INSTRUCTIONS.md)
6. **Consolidate RPC docs** (merge RPC_ARCHIVE_NODES.md + RPC_TESTING.md)
7. **Update links** (update all internal links to new paths)
8. **Remove old files** (only after links updated)
9. **Update README.md** (add new sections, update navigation)

## Validation

After reorganization, verify:
- ✅ All files in correct locations per structure
- ✅ All internal links updated and working
- ✅ No broken links
- ✅ All content preserved (no information loss)
- ✅ README.md provides clear navigation

