# Release examples and documentation edit

The operator requested the PR review's dependency-example fix and a plain-English
edit of the documentation. Update public installation examples to 0.3, then edit
the root and crate READMEs, tutorial, reference pages, and method guides. Remove
sales language, repetitive callouts, and unsupported blanket assurances. Keep
equations, reference data, API names, and reproducibility limits clear. Historical
plans, research reports, workspace doctrine, and prior journals retain their
original wording.

Check the resulting diff, dependency examples, local links, and code fences.
Run documentation tests and build rustdoc because crate READMEs supply the
crate-level API documentation. If benchmark prose changes, update its generator
too, preserving the recorded measurements.

The operator expanded the task to a literature audit, explicit error reports,
proposed fixes, and independent subagent review. Audit variance, distribution,
and surrogate methods in separate research lanes; check the remaining methods
in the owning lane. Keep research and a consolidated correction plan in papers/.
Correct documentation now; distinguish unimplemented algorithm fixes from
editorial corrections. Have a separate reviewer inspect the resulting changes
and proposals after integration.

Completed the public prose edit and literature audit. Updated release examples,
all nine method guides, eight crate READMEs, root README and documentation hub,
bibliography, and relevant source documentation. Recorded direct citations,
source/version caveats, and proposed implementation fixes in
`papers/2026-10-07-literature-audit.md` and four detailed lane reports. No
production algorithm or existing test assertion changed; no dependency added.

Used the operator's Stacks library on neuroses through read-only searches and
retained original PDFs. Checked actual title pages because several catalog
records attached incorrect titles or journal DOIs to conference versions.
Stopped orphaned task-started metadata scans after identifying their PIDs;
subsequent searches were bounded. Missing full texts are in
`papers/SHOPPING_LIST.md`. PDFs remain ignored.

Verification: 25 public pages and 36 registered citation destinations pass the
local checker. Online access returned 29 HTTP 200 and seven publisher HTTP 403;
access status is not semantic verification. All 27 doctests pass and workspace
rustdoc builds. The benchmark generator preserves every recorded timing row.
A line comparison confirmed existing Rust executable lines are unchanged.
Whitespace checks pass. New API-level HDMR, QOSA, tied-rank and spectral oracles
reproduce defects and exit nonzero; they do not imply those algorithms are fixed.
The initial PR review's 980 passing tests lacked these counterexamples.

A separate independent reviewer confirmed the main defect classifications and
found five remaining documentation/plan issues: sampled DGSM proof wording,
eFAST frequency description, derivative regularity assumptions, unverified
Hickernell equation references, and the distinction between rejecting unsupported
HDMR inputs and supporting them accurately. Corrected all five; reviewer
rechecked the current files and reported no remaining findings in its scope.
See `papers/2026-10-07-independent-review.md`.

No commit, push, merge, or external review comment was made. The working tree
contains the reviewable documentation correction and implementation-fix plan.
