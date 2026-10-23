# Coverage study blocked on infrastructure

2026-10-23: neuroses dropped off Tailscale ("offline, last seen" about 9
minutes before the check), along with the `go` host, after the first full
coverage run began. Both ssh and ping time out. That run used the
pre-review harness and is discarded in any case.

Blocked, pending the operator restoring neuroses:

- the full G-theory coverage study for #5 (90 cells, n_sim = 1900, B = 999,
  about 7 core-hours; `salib-gtheory-coverage full 1900 999 <out.md> <threads>
  [first_cell] [end_cell]` shards by cell range);
- the study report in `papers/`, and the docs scoped to the cells that pass;
- marking PR `fix/g-theory-bootstrap` ready.

Until the study runs, salib claims calibration for none of the new intervals.
The branch is complete apart from that: tests, the Sage oracles and three
review rounds are done. Work that needs no fleet continues (crolib-hmz
oracle targets).
