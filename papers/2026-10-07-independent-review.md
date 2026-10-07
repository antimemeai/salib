# Independent review, 2026-10-07

A separate subagent reviewed the combined working tree against HEAD, inspected
implementation paths behind the proposed fixes, and checked the documentation
validator. It did not participate in the original three research lanes.

Findings and resolution:

1. DGSM's `st_upper` field still claimed a sampled estimate proved a population
   bound. Replaced with plug-in estimate wording and explicit uncertainty.
2. The eFAST bibliography still used “all non-characteristic frequencies.”
   Replaced with the assigned low-frequency band.
3. The derivative guide needed weak differentiability and square-integrable
   derivatives, not square integrability of the model alone. Added assumptions.
4. The discrepancy table retained unverified exact paper equation references.
   Removed them and described the measures without unsupported attribution.
5. The HDMR plan conflated rejecting unsupported distributions with returning
   accurate triangular-input indices. Distinguished expected-error validation
   for an immediate rejection guard from the future supported-model oracle.

The reviewer confirmed the HDMR, tied-rank, QOSA and spectral defect
classifications against code. LARS/PRESS findings correctly distinguish a code
mismatch from an unquantified numerical consequence. Modified tracked Rust code
contains documentation/comment changes only. Local checks pass 25 pages and
36 registered citations.

After the corrections, the reviewer reread all five locations and reported no
remaining findings from this review. This is a scoped review, not a certification
of the entire library; production defects and acquisition gaps remain in the
[consolidated audit](2026-10-07-literature-audit.md).
