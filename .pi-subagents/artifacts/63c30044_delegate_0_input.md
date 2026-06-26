# Task for delegate

Deep-dive review of the Rust crate "grixy" at /Users/matan/Developer/grixy.

Do all of the following:

1. **Review the crate in its entirety.** Read Cargo.toml, README.md, CHANGELOG.md, and every source file under src/ (including submodules like buf/, ops/, transform/, internal.rs, core.rs, prelude.rs, test.rs). Understand the trait hierarchy (GridBase, ExactSizeGrid, TrustedSizeGrid, GridRead, GridWrite, GridIter, GridDiff, unchecked variants), the feature flags (alloc, buffer, cell, serde), the sealed-trait pattern, and how transforms (Copied, Mapped, Viewed, Scaled, Blended) work. Also check tests and benches for usage patterns. Note the crate depends on "ixy" (layout/traversal crate) — check its role too if source is available via cargo registry cache (e.g. `find ~/.cargo -path '*ixy-0.6*' -name '*.rs'` or similar) — otherwise infer from usage.

2. **Research similar/comparable crates and their design** for 2D grid abstractions in Rust and other ecosystems relevant to embedded/graphics/gamedev use cases. Look at crates like `grid`, `ndarray` (2D case), `array2d`, `grid_2d`, `bevy`'s grid/tilemap crates, `nalgebra` (for matrix parallels), and any embedded-graphics adjacent crates (e.g. `embedded-graphics` framebuffer abstractions). Use web_search for "rust 2d grid crate", "rust no_std 2d array crate", "embedded-graphics framebuffer trait design", etc. Compare their trait design, indexing API, iterator design, no_std support, generic parameterization, and ergonomics vs grixy's approach.

3. **Recommend concrete API, architectural, and internal code improvements.** Assume breaking changes are fully allowed (pre-1.0, alpha version). Be specific and grounded in the actual code you read — cite file paths and line-level context, not generic advice. Cover things like:
   - Trait hierarchy simplification/consolidation opportunities
   - Naming inconsistencies or confusing API surface
   - Missing ergonomic conveniences (indexing operators, iterator adapters, common constructors)
   - Opportunities to reduce unsafe surface area or improve the unchecked/checked trait split
   - Feature flag organization
   - Places where grixy diverges from Rust API Guidelines (check for `#[must_use]`, doc examples, trait bound placement per project's own AGENTS.md rules)
   - Performance considerations (allocations, iterator overhead, monomorphization bloat)
   - Comparison-driven suggestions: "crate X does Y, grixy could benefit from a similar approach because Z"
   - Anything about the ixy dependency boundary/coupling worth reconsidering

Write the full findings and recommendations as a well-structured markdown document to /Users/matan/Developer/grixy/.matan/improve.md. Structure it with: an Executive Summary, a Crate Overview section, a Comparative Landscape section (with a summary table of comparable crates), and a Recommendations section organized by priority (High/Medium/Low) with concrete before/after code snippets where useful and file:line references. Do not hold back — be thorough and opinionated, this is an internal design review document, not user-facing docs.

At the end of your turn, just confirm the file was written and give a short 3-5 sentence summary of the top 3 recommendations.

## Acceptance Contract
Acceptance level: checked
Completion is not accepted from prose alone. End with a structured acceptance report.

Criteria:
- criterion-1: Implement the requested change without widening scope

Required evidence: changed-files, tests-added, commands-run, residual-risks, no-staged-files

Finish with a fenced JSON block tagged `acceptance-report` in this shape:
Use empty arrays when no items apply; array fields contain strings unless object entries are shown.
```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "specific proof"
    }
  ],
  "changedFiles": [
    "src/file.ts"
  ],
  "testsAddedOrUpdated": [
    "test/file.test.ts"
  ],
  "commandsRun": [
    {
      "command": "command",
      "result": "passed",
      "summary": "short result"
    }
  ],
  "validationOutput": [
    "validation output or concise summary"
  ],
  "residualRisks": [
    "none"
  ],
  "noStagedFiles": true,
  "diffSummary": "short description of the diff",
  "reviewFindings": [
    "blocker: file.ts:12 - issue found, or no blockers"
  ],
  "manualNotes": "anything else the parent should know"
}
```