## tracing-unwrap
This crate provides `.unwrap_or_log()` and `.expect_or_log()` methods on `Result` and `Option` types that log failed unwraps to a [`tracing::Subscriber`]. This is useful when, for example, you are logging to syslog or a database, and you want your unwrap failures to show up there instead of being printed to `stderr`.

Its API aims to mirror Rust's `std` — see all the [supported methods](#methods) below. Failed unwraps are logged at a level of [`ERROR`].

[![crates.io](https://img.shields.io/crates/v/tracing-unwrap?label=latest)](https://crates.io/crates/tracing-unwrap)
[![Documentation](https://docs.rs/tracing-unwrap/badge.svg)](https://docs.rs/tracing-unwrap)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/abreis/tracing-unwrap)

### Usage
Add the following to your `Cargo.toml`:
```toml
tracing-unwrap = "1.0"
```

Next, bring the [`ResultExt`] and/or [`OptionExt`] traits into scope, and make use of the new logging methods.
```rust
use tracing_unwrap::ResultExt;

tracing_subscriber::fmt().init();
let not_great: Result<(), _> = Result::Err("not terrible");

// Logs the failed unwrap and panics
not_great.unwrap_or_log();
```

### Methods
| `std` method                             | `tracing-unwrap` form               | trait         |
| ---------------------------------------- | ----------------------------------- | ------------- |
| [`Result::ok()`]                         | [`Result::ok_or_log()`]             | [`ResultExt`] |
| [`Result::unwrap()`]                     | [`Result::unwrap_or_log()`]         | [`ResultExt`] |
| [`Result::expect(msg)`]                  | [`Result::expect_or_log(msg)`]      | [`ResultExt`] |
| [`Result::unwrap_err()`]                 | [`Result::unwrap_err_or_log()`]     | [`ResultExt`] |
| [`Result::expect_err(msg)`]              | [`Result::expect_err_or_log(msg)`]  | [`ResultExt`] |
| [`Option::unwrap()`]                     | [`Option::unwrap_or_log()`]         | [`OptionExt`] |
| [`Option::expect(msg)`]                  | [`Option::expect_or_log(msg)`]      | [`OptionExt`] |
| [`Option::unwrap_none()`]<sup>†</sup>    | [`Option::unwrap_none_or_log()`]    | [`OptionExt`] |
| [`Option::expect_none(msg)`]<sup>†</sup> | [`Option::expect_none_or_log(msg)`] | [`OptionExt`] |

_†: no longer in `std`, see [`rust-lang/rust#62633`](https://github.com/rust-lang/rust/issues/62633)_<br/>

### Features
* **`panic-quiet`**: causes failed unwraps to panic with an empty message.<br/>
  This feature is enabled by default — if you'd like the unwrap error message to also show in the panic message, disable default features in your `Cargo.toml` as follows:<br/>
  `tracing-unwrap = { version = "1.0", default-features = false }`

* **`log-location`**: calls [`std::panic::Location::caller()`] to determine the location of a failed unwrap.

[`tracing::Subscriber`]: https://docs.rs/tracing/*/tracing/trait.Subscriber.html
[`ResultExt`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html
[`OptionExt`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html
[`ERROR`]: https://docs.rs/tracing/*/tracing/struct.Level.html#associatedconstant.ERROR
[`Result::ok()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
[`Result::unwrap()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
[`Result::expect(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[`Result::unwrap_err()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err
[`Result::expect_err(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err
[`Option::unwrap()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
[`Option::expect(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
[`Option::unwrap_none()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_none
[`Option::expect_none(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect_none
[`Result::ok_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.ok_or_log
[`Result::unwrap_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.unwrap_or_log
[`Result::expect_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.expect_or_log
[`Result::unwrap_err_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.unwrap_err_or_log
[`Result::expect_err_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.expect_err_or_log
[`Option::unwrap_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.unwrap_or_log
[`Option::expect_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.expect_or_log
[`Option::unwrap_none_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.unwrap_none_or_log
[`Option::expect_none_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.expect_none_or_log
[`std::panic::Location::caller()`]: https://doc.rust-lang.org/std/panic/struct.Location.html#method.caller
