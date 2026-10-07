# salib-cli

The command-line interface is not implemented yet. The binary prints
`salib: CLI not yet implemented` and exits with status 2. Use the `salib`
library to run analyses.

## Current behavior

From a source checkout:

```sh
cargo run -p salib-cli --bin salib
# salib: CLI not yet implemented
# Exit status: 2
```

This package has no working subcommands, public library API, or optional
features. For sampling and analysis, see `salib-samplers` and `salib-estimators`.

[Library tutorial](https://github.com/antimemeai/salib/blob/master/docs/quickstart.md).
MIT OR Apache-2.0.
