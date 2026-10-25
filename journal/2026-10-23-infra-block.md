# Coverage study blocked on infrastructure

2026-10-23: neuroses dropped off Tailscale (offline since about 09:00 UTC),
along with the `go` host. ssh and ping time out over Tailscale. Its LAN
addresses on 192.168.50.0/24 (192.168.50.100 and .106, matched by host key)
also do not answer, and no host on that /24 has SSH open, so it is off the
LAN as well, not merely re-addressed.

Correction: the coverage run never started. The ssh session that cloned the
branch (63c5d0b) dropped while `cargo build --release` was still compiling,
and `coverage-full.log` was never created. Nothing on neuroses needs
discarding except possibly an interrupted build in
~/salib-coverage-2026-10-23, which will be re-cloned.

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
