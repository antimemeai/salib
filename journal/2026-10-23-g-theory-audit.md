# G-theory audit against crolib's corpus

The operator set a standing rule: fix salib bugs and ambiguities as they are
found, or at least open issues for them. salib keeps its small crossed p×i×r
G-theory; crolib owns the deeper theory. Beads has no database here, so GitHub
issues are the tracker.

Reviewed `salib-estimators/src/g_theory.rs` against crolib's design and research
lanes. Confirmed the checkout's `g_theory.rs` is byte-identical to the published
`salib-estimators` 0.3.0 crate. Ran every reproduction against that code from an
out-of-repository scratch crate; no salib source changed.

- #3: components are signed EMS estimates, but the docs say "clamped at zero" and
  "nonnegative". G/Phi leave [0, 1]: one 3×3×2 grid gives G = 2.25; of 19,690
  random grids, 4,403 give G > 1 and 9,302 give Phi > G. The coefficient policy
  for negative components needs an operator decision.
- #4: absolute 1e-15 thresholds. The gate1 grid ×1e-8 silently returns G = Phi = 0;
  ×1e-9 returns `ZeroVariance`. 0/0 relative G is reported as 0, and an existing
  test locks that value in.
- #5: the boot-p,i,r replicate distribution is biased (20×8×4, all components 1,
  200×200: σ²(p) +0.328, σ²(pir) −0.377, MCSE ≤ 0.016). Percentiles use only
  successful draws.
- #6: the gate1 "textbook" fixture is self-derived. salib already reproduces
  Brennan (2001) Synthetic Data Set No. 3, as transcribed by GeneralizIT
  8af4fae, to all printed digits. The point-estimate EMS algebra is correct.

Issue #2 is an unrelated outside link and was left alone.
