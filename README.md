# Testrunner #

I started this project in the summer of 2017 under the mentorship of Brian Anderson (co-creator, Rust),
originally planned to become a part of the [stx](https://github.com/brson/stdx) project [Issue #36](https://github.com/brson/stdx/issues/36). Unfortunately, Brian is not actively maintaing Rust anymore.

Nevertheless, this project was a great way for me to learn about Rust and build scripts.

### Aim of the project ###

This project autogenerates tests for each crate in Cargo-stdx.toml, and subsequently tests each crate.
To learn more about build scripts, check out here: [build scripts](http://doc.crates.io/build-script.html).

To build the scripts and generate tests:

```rust
cargo build
```

To test the crates:

```rust
cargo test
```

