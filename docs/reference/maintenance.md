# Documentation Maintenance Guide

**Purpose**: Guide for maintainers on how to update documentation when adding new features or making changes.

## Overview

This guide explains the documentation structure and provides a systematic approach to updating documentation when adding features, fixing bugs, or making changes to the codebase.

## Documentation Structure

### Organization Pattern

Documentation follows a **hybrid pattern**:
- **README.md** at root: Entry point with overview, quick start, and navigation
- **docs/** directory: Detailed documentation organized by topic

### Directory Structure

```
docs/
├── api/                    # API documentation
│   ├── rest-api.md        # REST API server
│   └── programmatic-api.md # Library API
├── guides/                 # User guides
│   ├── algorithms.md      # Algorithm extensibility
│   ├── performance.md     # Performance benchmarks
│   └── rpc-usage.md       # RPC endpoints and archive nodes
├── polkadot/              # Polkadot ecosystem context
│   └── ecosystem-overview.md
├── reference/             # Reference material
│   ├── glossary.md        # Technical terms
│   ├── maintenance.md    # This file
│   └── rfp-compliance.md  # RFP compliance assessment
└── testing/               # Testing documentation
    └── overview.md        # Testing guide and examples
```

## Update Procedures

### Adding a New Algorithm

When adding a new election algorithm:

1. **Update Algorithm Guide** (`docs/guides/algorithms.md`)
   - Add algorithm to "Current Algorithms" section
   - Add implementation example
   - Update architecture documentation if needed

2. **Update REST API Documentation** (`docs/api/rest-api.md`)
   - Add algorithm to algorithm list
   - Add REST API examples for new algorithm
   - Update synthetic data examples if needed

3. **Update Programmatic API Documentation** (`docs/api/programmatic-api.md`)
   - Add algorithm to `AlgorithmType` enum documentation
   - Add programmatic API examples
   - Update code examples

4. **Update Testing Documentation** (`docs/testing/overview.md`)
   - Add algorithm to "Available Algorithms" section
   - Add test examples if applicable

5. **Update README.md**
   - Add algorithm to features list
   - Update algorithm comparison examples if needed

6. **Update Glossary** (`docs/reference/glossary.md`)
   - Add algorithm term if it's a new type

### Adding a New API Endpoint

When adding a new REST API endpoint:

1. **Update REST API Documentation** (`docs/api/rest-api.md`)
   - Document endpoint in appropriate section
   - Add request/response examples
   - Add error handling documentation

2. **Update Programmatic API Documentation** (`docs/api/programmatic-api.md`)
   - Add corresponding library API if applicable
   - Update examples

3. **Update Testing Documentation** (`docs/testing/overview.md`)
   - Add test examples for new endpoint

### Adding a New Data Source

When adding support for a new data source (e.g., new RPC method):

1. **Update RPC Usage Guide** (`docs/guides/rpc-usage.md`)
   - Document new data source
   - Add examples
   - Update troubleshooting section

2. **Update API Documentation**
   - Update REST API docs (`docs/api/rest-api.md`)
   - Update programmatic API docs (`docs/api/programmatic-api.md`)

3. **Update Testing Documentation** (`docs/testing/overview.md`)
   - Add test examples

### Updating Technical Terms

When introducing new technical terms:

1. **Update Glossary** (`docs/reference/glossary.md`)
   - Add term definition
   - Categorize appropriately
   - Add cross-references

2. **Add Inline Definitions**
   - Add definition where term first appears in each document
   - Link to glossary

3. **Update Related Documentation**
   - Ensure consistent usage across all docs

## Cross-Reference Maintenance

### Adding Cross-References

When adding cross-references between related documentation:

1. **Identify Related Topics**
   - Algorithms ↔ API docs
   - Testing ↔ Guides
   - RPC ↔ API docs

2. **Add "Related Documentation" Section**
   - Add at the top of relevant documents
   - Keep links current

3. **Update When Structure Changes**
   - Update cross-references when files move
   - Verify links still work

## Link Verification

### Before Committing

Always verify links before committing documentation changes:

1. **Check Internal Links**
   - Verify all `[text](path)` links point to existing files
   - Verify section anchors work (`#section-name`)

2. **Check External Links**
   - Verify external URLs still work
   - Update if resources moved

3. **Check Cross-References**
   - Verify "Related Documentation" sections are current
   - Update if files moved or renamed

### Tools for Link Verification

```bash
# Find all markdown links
grep -r "\[.*\](.*\.md)" --include="*.md" docs/

# Check for broken internal links (manual verification required)
# Test each link by navigating to the file
```

## Content Preservation

### During Consolidation

When consolidating or moving files:

1. **Preserve All Content**
   - Ensure no information is lost
   - Merge overlapping content carefully
   - Preserve unique sections

2. **Update All Links**
   - Update links before removing old files
   - Verify links work after consolidation

3. **Maintain Examples**
   - Preserve all code examples
   - Update file paths in examples if needed

## Documentation Standards

### Writing Style

- **Clear and Concise**: Use clear language, avoid jargon
- **Examples First**: Provide examples early in documentation
- **Progressive Disclosure**: Start with simple examples, add complexity
- **Consistent Formatting**: Use consistent markdown formatting

### Code Examples

- **Complete Examples**: Provide complete, runnable examples
- **Comments**: Add comments explaining key parts
- **Error Handling**: Show error handling when relevant
- **Multiple Formats**: Show both JSON and human-readable formats when applicable

### Terminology

- **Consistent Terms**: Use consistent terminology across all docs
- **Define First Use**: Define terms where first used
- **Link to Glossary**: Link to glossary for comprehensive definitions

## Checklist for Documentation Updates

When updating documentation:

- [ ] Identify all affected documentation files
- [ ] Update primary documentation file(s)
- [ ] Update cross-references in related files
- [ ] Update README.md navigation if structure changes
- [ ] Update glossary if new terms introduced
- [ ] Verify all links work correctly
- [ ] Ensure examples are current and accurate
- [ ] Preserve all existing content (no information loss)
- [ ] Update "Last Updated" dates if applicable
- [ ] Review for consistency and clarity

## Common Update Scenarios

### Scenario 1: Adding a New Feature

1. Identify affected documentation files
2. Update primary documentation
3. Update related documentation with cross-references
4. Update examples
5. Update glossary if needed
6. Verify links

### Scenario 2: Fixing a Bug

1. Update relevant documentation if behavior changed
2. Update examples if they were incorrect
3. Update troubleshooting sections if applicable

### Scenario 3: Refactoring Code

1. Update code examples in documentation
2. Update file paths if structure changed
3. Update cross-references if files moved
4. Verify all links still work

### Scenario 4: Updating Dependencies

1. Update version numbers in examples if applicable
2. Update compatibility information
3. Update installation instructions if needed

## Related Documentation

- [Documentation Structure](../../README.md#documentation-structure) - Overview of documentation organization
- [Glossary](glossary.md) - Technical terms definitions
- [Testing Overview](../testing/overview.md) - Testing documentation

## Questions?

If you're unsure about documentation updates:

1. Check this guide first
2. Review similar updates in git history
3. Check related documentation for patterns
4. Ask maintainers for guidance

