# Phase 6: Stateright protocol models

## Implementation plan

Implement four finite models with independent symbolic oracles, production API
calls, safety properties, and reachable success/error witnesses. Problem drafts
hold up to three factors. RNG handles have two salts and at most three drawn
words each, with a recorded checkpoint separate from the live position. Saltelli
assembly explores both orders over two rows and two factors using tagged cells
from a concrete symbolic sampler. Estimator collection covers one/two factors,
two/three rows, duplicate/stale/invalid arrivals, and ordinary/constant outputs.

The estimator model specifies the prospective asynchronous collection boundary;
the current estimator is synchronous. Only complete, correctly shaped cached
outputs are sent to the real estimator. Publication is a separate transition.

Run exhaustive Stateright BFS with safety and reachability properties, then the
requested crate tests, binary, Clippy, and workspace tests. Report unique states,
generated transitions, depth, and any counterexamples.

## API and dependency adaptations

The requested `stateright = "2"` is unavailable: the published/cached release is
0.31.0. Use `stateright = "0.31"` and the supported
`model.checker().spawn_bfs().join()` API, followed by `assert_properties()`.
There is no `Model::bfs`, `initial_states`, or `ExploreResult` in this release.
See the [Model documentation](https://docs.rs/stateright/0.31.0/stateright/trait.Model.html).
Stateright is a regular dependency because the server binary uses it.

Add the existing workspace dependencies `rand` (drawing real ChaCha words),
`ndarray` (the symbolic sampler's trait return type), and `salib-estimators`
(checking real cached-output numerical behavior). Add workspace package version
0.2.0 so the requested `version.workspace = true` can resolve.

`validate_distribution` is private. Successful distributions are revalidated
through a one-factor `ProblemBuilder`, which calls that validator; no production
visibility change is needed. Current production code rejects nonfinite Normal
means and guards zero variance, unlike the earlier analysis document.

## Results

All four models passed exhaustive BFS without assertion failures or safety
counterexamples. Every declared `sometimes` witness was found. Counts include
both ordinary and adversarial action orders within the bounds below.

| Model | Unique states | Generated transitions | Action depth | Safety properties | Reachability properties | Result |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Problem builder | 3,770 | 3,769 | 4 | 4 | 4 | PASS |
| RNG protocol | 2,400 | 23,800 | 8 | 3 | 4 | PASS |
| Saltelli assembly | 28 | 46 | 7 | 4 | 2 | PASS |
| Estimator completeness | 144,364 | 3,660,978 | 14 | 6 | 8 | PASS |

Transitions are Stateright's generated-state count minus the initial-state
count, including repeated destinations and self loops. Depth subtracts one
from Stateright's state-path depth, so initial states have action depth zero.
No timeout, state limit, depth limit, or boundary pruning is configured. Safety
properties keep successful checking active until the state space is exhausted;
finding all reachability witnesses cannot stop it early. Stateright deduplicates
states by fingerprints, as its standard checker does.

### Checked invariants and domains

- **Problem builder:** 0–3 ordered factors, names A/B/C, valid Uniform and Normal,
  reversed Uniform, and NaN-mean Normal. Successful builds are nonempty, names
  are unique, and distributions pass validation. The real result is checked
  against independent symbolic precedence: empty, first duplicate, first invalid
  distribution, success. Transitions also check factor order and revalidate each
  successful distribution through the public builder. Success, empty errors,
  duplicates combined with invalid distributions, and NaN rejection are reachable.
- **RNG protocol:** seeds 0/0x42, root streams 0/7, two fixed salts, fork depth
  one, three handles, one optional parent checkpoint, and cumulative draw counts
  0–3 per handle. Transitions assert deterministic forks, unchanged parent,
  zero-position children, inherited seed/algorithm, and position-independent fork
  identity. Every draw and resumed lookahead is compared with an independent
  zero-origin stream advanced by real draws. Snapshots preserve identity and
  position. Checkpoints can lag the live parent, repeated salts reset children,
  children draw independently, and resume interleaves with draws and forks.
- **Saltelli assembly:** d=2, n=2, first/total and second-order modes. A custom
  production `Sampler` emits eight distinct exact tags, identifying half, row,
  and column. The real `build_saltelli_matrix` produces components; the model
  commits base halves in either order and hybrids in every legal order. All base
  and hybrid cells match their independent provenance oracle, shapes and hybrid
  counts match, completion requires every requested hybrid, and real evaluation
  counts equal 8 or 12. Both modes reach completion.
- **Estimator completeness:** first/total campaigns cover d=1–2 and n=2–3;
  second-order covers d=2/n=2. Every configuration has ordinary integer outputs
  and all-one outputs. All individual slot arrival orders and duplicate retries
  are explored. Stale campaign IDs, invalid hybrid indices, and out-of-range rows
  cannot fill slots. Series of length n−1/n+1 permanently taint the campaign.
  Estimation requires all identities, including BA in second-order mode, and
  correct lengths. Real cached-output estimators return finite, correctly shaped
  results; constant campaigns return zeros, including second-order indices.
  Publication requires a complete untainted campaign and a validated result.

The additional direct estimator test verifies the synchronous APIs' documented
panic rejection of missing/short/long B, AB, and BA series, empty hybrids, and
wrong BA hybrid counts. Both cached-output entry points are exercised on their
independent validation branches. Expected caught panics are rejection evidence,
not invariant violations.

### Scope

The collector's asynchronous identity/publication guarantees describe a proposed
protocol, not an existing production orchestration API. Numerical checks call
production estimators on complete arrays; direct rejection tests separately check
their shape boundary. Symbolic Saltelli states represent commitment of components
from the synchronous production builder, rather than exposing its local variables.
Groups, arbitrary deserialization, wrong-seed snapshots, noncanonical RNG
positions, nested forks, nonfinite estimator outputs, and bootstrap scheduling
are outside these bounded harnesses. No stream uniqueness or statistical
independence claim is made.

### Review and verification journal

Independent plan review required nonvacuous success/error witnesses, production
provenance comparisons, an independently advanced RNG reference, and second-order
completeness. All are included. Independent code review found missing direct
second-order B/AB-length and empty AB/BA rejection cases; those cases were added.
No remaining code-review defect was reported.

The registry cannot resolve from this sandbox; verification uses the installed
dependency cache with `--offline --locked`. Unrelated lockfile package versions
were preserved. The regular Stateright dependency adds its own transitive packages
and enables the rand features it requires.

- `cargo test --offline --locked -p salib-models`: PASS, five tests.
- `cargo run --offline --locked -p salib-models`: PASS, statistics above.
- `cargo clippy --offline --locked -p salib-models --all-targets -- -D warnings`: PASS.
- `cargo test --offline --locked --workspace`: PASS, 953 tests across 70 test groups,
  no failures or ignored tests. The final run includes the expanded rejection cases.

Run the same commands without the offline/locked switches on the server if it
needs to fetch dependencies. `cargo run -p salib-models` fails with a nonzero exit
on any checker panic, safety counterexample, or missing reachability witness.
