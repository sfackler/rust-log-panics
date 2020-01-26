# rust-log-panics

[![CircleCI](https://circleci.com/gh/sfackler/rust-log-panics.svg?style=shield)](https://circleci.com/gh/sfackler/rust-log-panics)

[Documentation](https://docs.rs/log-panics)

A panic hook which logs panics rather than printing them.

## Logging with a backtrace

Because logging with a backtrace requires additional dependencies, the `with-backtrace` feature must be enabled in order to use it. You can add the following in your `Cargo.toml`:

```
log-panics = { version = "2", features = ["with-backtrace"]}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
