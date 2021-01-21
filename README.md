## What?

This is a set of a Rust target custom file + a little test library for developing Master Boot Records (MBRs) directly in Rust.

## Was this necessary?

No. Absolutely not. But is fun.

## How?

You might take a look at the example project located in the
[examples](mbr-rs/example) folder. Also, if you're creating a new
independent project, make sure you configure your Cargo project for
compile using the custom
[target](mbr-target/i386-unknown-none-mbr.json) provided in this
repository, as it is defined in the [workspace
configuration](mbr-rs/.cargo/config.toml).
