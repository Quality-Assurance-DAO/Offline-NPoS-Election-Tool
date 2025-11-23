# Redundancy Analysis: Root-Level Markdown Files

**Created**: 2025-01-27  
**Purpose**: Analyze root-level markdown files for redundancies (>60% overlap) and identify unique content to preserve

## Analysis Method

Files were analyzed by:
1. Reading full content of each file
2. Identifying overlapping topics and content
3. Determining unique content in each file
4. Calculating approximate overlap percentage

## Results

### Files with Significant Overlap (>60%)

| File 1 | File 2 | Overlap % | Unique Content to Preserve | Action |
|--------|--------|-----------|---------------------------|--------|
| TESTING.md | TEST_RUNNING_INSTRUCTIONS.md | ~60% | Both have unique sections | Consolidate into `docs/testing/overview.md` |
| RPC_ARCHIVE_NODES.md | RPC_TESTING.md | ~65% | Both have unique sections | Consolidate into `docs/guides/rpc-usage.md` |

## Detailed Analysis

### 1. TESTING.md + TEST_RUNNING_INSTRUCTIONS.md

**Overlap**: ~60%  
**Status**: REDUNDANT - Should be consolidated

#### Overlapping Content
- Both cover test execution
- Both mention cargo test commands
- Both provide troubleshooting guidance
- Both reference test data formats

#### Unique Content in TESTING.md
- Focus on running election simulations (CLI usage)
- Test data JSON format examples
- Expected output formats (JSON, human-readable)
- Election-specific troubleshooting
- Example test data structure

#### Unique Content in TEST_RUNNING_INSTRUCTIONS.md
- Comprehensive cargo test command reference
- Test types breakdown (unit, integration, edge cases, performance)
- Debugging techniques (backtrace, verbose output)
- CI/CD integration examples
- Test coverage generation
- Continuous testing with cargo-watch

#### Consolidation Strategy
**Target**: `docs/testing/overview.md`

**Structure**:
1. **Quick Start** (from TESTING.md)
   - Build the tool
   - Prepare test data
   - Run an election

2. **Test Data Format** (from TESTING.md)
   - JSON structure
   - Example test data

3. **Expected Output** (from TESTING.md)
   - Human-readable format
   - JSON format

4. **Running Tests** (from TEST_RUNNING_INSTRUCTIONS.md)
   - Run all tests
   - Test types
   - Running specific test categories

5. **Test Commands Reference** (from TEST_RUNNING_INSTRUCTIONS.md)
   - Comprehensive command reference
   - Filtering and debugging

6. **Troubleshooting** (merge from both)
   - Election-specific issues (from TESTING.md)
   - Test execution issues (from TEST_RUNNING_INSTRUCTIONS.md)

**Content Preservation**: ✅ All unique content will be preserved

### 2. RPC_ARCHIVE_NODES.md + RPC_TESTING.md

**Overlap**: ~65%  
**Status**: REDUNDANT - Should be consolidated

#### Overlapping Content
- Both mention archive nodes
- Both list RPC endpoints
- Both cover historical block queries
- Both provide troubleshooting guidance

#### Unique Content in RPC_ARCHIVE_NODES.md
- Comprehensive archive node explanation
- Detailed error handling documentation
- Best practices for historical queries
- Pre-fetching and caching strategies
- Detailed troubleshooting scenarios
- Alternative approaches (pre-fetched JSON)

#### Unique Content in RPC_TESTING.md
- Quick test guide format
- Current implementation status
- Next steps for full RPC support
- Quick reference for testing connection
- Verification steps

#### Consolidation Strategy
**Target**: `docs/guides/rpc-usage.md`

**Structure**:
1. **Overview** (from RPC_ARCHIVE_NODES.md)
   - What are archive nodes
   - Why archive nodes are needed

2. **Quick Start** (from RPC_TESTING.md)
   - Quick test commands
   - Connection verification

3. **Supported RPC Endpoints** (merge from both)
   - Archive node providers
   - Regular RPC endpoints
   - How to verify archive node support

4. **Historical Block Queries** (from RPC_ARCHIVE_NODES.md)
   - How the tool handles historical queries
   - RPC methods used
   - Error handling

5. **Best Practices** (from RPC_ARCHIVE_NODES.md)
   - Using archive node endpoints
   - Pre-fetching and caching
   - Verifying block numbers
   - Handling rate limits

6. **Troubleshooting** (merge from both)
   - Common problems and solutions
   - Error messages and meanings

7. **Current Status** (from RPC_TESTING.md)
   - What's working
   - What's in progress
   - Next steps

**Content Preservation**: ✅ All unique content will be preserved

## Other Files

### API_USAGE.md
- **Status**: UNIQUE - No significant overlap
- **Action**: Split into `docs/api/rest-api.md` and `docs/api/programmatic-api.md` (as planned)

### ALGORITHM_EXTENSIBILITY.md
- **Status**: UNIQUE - No overlap
- **Action**: Move to `docs/guides/algorithms.md`

### PERFORMANCE_BENCHMARKS.md
- **Status**: UNIQUE - No overlap
- **Action**: Move to `docs/guides/performance.md`

### RFP_COMPLIANCE_ASSESSMENT.md
- **Status**: UNIQUE - No overlap
- **Action**: Move to `docs/reference/rfp-compliance.md`

### README.md
- **Status**: UNIQUE - Entry point, references other files
- **Action**: Enhance with new content (as planned)

## Summary

- **Total files analyzed**: 9 root-level markdown files
- **Redundant pairs found**: 2 pairs
- **Files with >60% overlap**: 4 files (2 pairs)
- **Action required**: Consolidate 2 pairs into 2 target files

## Consolidation Plan

1. **TESTING.md + TEST_RUNNING_INSTRUCTIONS.md** → `docs/testing/overview.md`
   - Preserve all unique content
   - Organize by topic sections
   - Merge overlapping content

2. **RPC_ARCHIVE_NODES.md + RPC_TESTING.md** → `docs/guides/rpc-usage.md`
   - Preserve all unique content
   - Organize by topic sections
   - Merge overlapping content

## Next Steps

Proceed to outdated file analysis (T006) to identify files referencing non-existent features or contradicted content.

