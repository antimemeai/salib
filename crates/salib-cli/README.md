# salib-cli

Reserved command-line package for the salib workspace. The binary is currently a
stub: it prints `salib: CLI not yet implemented` and exits with status 2. Depend
on the `salib` library to run analyses today. There are no public library types,
functions, or working `sample`, `run`, or `analyze` subcommands in this package.

## Current behavior

From a source checkout:

```sh
cargo run -p salib-cli --bin salib
# salib: CLI not yet implemented
# Exit status: 2
```

For Rust applications use `salib`; for independent sampling or cached-data
analysis use `salib-samplers` or `salib-estimators`. The CLI package currently has
no optional features. Its library target is empty and is not an analysis API.

[Library tutorial](https://github.com/antimeme-ai/salib/blob/main/docs/quickstart.md).
MIT OR Apache-2.0.
