//! This crate provides `.unwrap_or_log()` and `.expect_or_log()` methods on `Result` and `Option` types that log failed unwraps to a [`tracing::Subscriber`]. This is useful when, for example, you are logging to syslog or a database, and you want your unwrap failures to show up there instead of being printed to `stderr`.
//!
//! Its API aims to mirror Rust's `std` — see all the [supported methods](#methods) below. Failed unwraps are logged at a level of [`ERROR`].
//!
//! [![crates.io](https://img.shields.io/crates/v/tracing-unwrap?label=latest)](https://crates.io/crates/tracing-unwrap)
//! [![Documentation](https://docs.rs/tracing-unwrap/badge.svg)](https://docs.rs/tracing-unwrap)
//! [![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/abreis/tracing-unwrap)
//!
//! ### Usage
//! Add the following to your `Cargo.toml`:
//! ```toml
//! tracing-unwrap = "0.10"
//! ```
//!
//! Next, bring the [`ResultExt`] and/or [`OptionExt`] traits into scope, and make use of the new logging methods.
//! ```ignore
//! use tracing_unwrap::ResultExt;
//!
//! tracing_subscriber::fmt().init();
//! let not_great: Result<(), _> = Result::Err("not terrible");
//!
//! // Logs the failed unwrap and panics
//! not_great.unwrap_or_log();
//! ```
//!
//! ### Methods
//! | `std` method                   | `tracing-unwrap` form               | trait         |
//! |--------------------------------| ----------------------------------------|---------------|
//! | [`Result::ok()`]               | [`Result::ok_or_log()`]               | [`ResultExt`] |
//! | [`Result::unwrap()`]           | [`Result::unwrap_or_log()`]           | [`ResultExt`] |
//! | [`Result::expect(msg)`]        | [`Result::expect_or_log(msg)`]        | [`ResultExt`] |
//! | [`Result::unwrap_err()`]       | [`Result::unwrap_err_or_log()`]       | [`ResultExt`] |
//! | [`Result::expect_err(msg)`]    | [`Result::expect_err_or_log(msg)`]    | [`ResultExt`] |
//! | [`Option::unwrap()`]           | [`Option::unwrap_or_log()`]           | [`OptionExt`] |
//! | [`Option::expect(msg)`]        | [`Option::expect_or_log(msg)`]        | [`OptionExt`] |
//! | [`Option::unwrap_none()`]<sup>†</sup>      | [`Option::unwrap_none_or_log()`]      | [`OptionExt`] |
//! | [`Option::expect_none(msg)`]<sup>†</sup>   | [`Option::expect_none_or_log(msg)`]   | [`OptionExt`] |
//!
//! *†: no longer in `std`, see [`rust-lang/rust#62633`](https://github.com/rust-lang/rust/issues/62633)*<br/>
//!
//!
//! ### Features
//! * **`panic-quiet`**: causes failed unwraps to panic with an empty message.<br/>
//!   This feature is enabled by default — if you'd like the unwrap error message to also show in the panic message, disable default features in your `Cargo.toml` as follows:<br/>
//!   `tracing-unwrap = { version = "0.10", default-features = false }`
//!
//! * **`log-location`**: calls [`std::panic::Location::caller()`] to determine the location of a failed unwrap.
//!
//! [`tracing::Subscriber`]: https://docs.rs/tracing/*/tracing/trait.Subscriber.html
//! [`ResultExt`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html
//! [`OptionExt`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html
//! [`ERROR`]: https://docs.rs/tracing/*/tracing/struct.Level.html#associatedconstant.ERROR
//! [`Result::ok()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.ok
//! [`Result::unwrap()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
//! [`Result::expect(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
//! [`Result::unwrap_err()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err
//! [`Result::expect_err(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err
//! [`Option::unwrap()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
//! [`Option::expect(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
//! [`Option::unwrap_none()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_none
//! [`Option::expect_none(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect_none
//! [`Result::unwrap_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.unwrap_or_log
//! [`Result::expect_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.expect_or_log
//! [`Result::unwrap_err_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.unwrap_err_or_log
//! [`Result::expect_err_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.ResultExt.html#tymethod.expect_err_or_log
//! [`Option::unwrap_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.unwrap_or_log
//! [`Option::expect_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.expect_or_log
//! [`Option::unwrap_none_or_log()`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.unwrap_none_or_log
//! [`Option::expect_none_or_log(msg)`]: https://docs.rs/tracing-unwrap/*/tracing_unwrap/trait.OptionExt.html#tymethod.expect_none_or_log
//! [`std::panic::Location::caller()`]: https://doc.rust-lang.org/std/panic/struct.Location.html#method.caller

use std::fmt;

//
// Extension trait for Result types.
//

/// Extension trait for Result types.
pub trait ResultExt<T, E> {
    /// Converts from `Result<T, E>` to [`Option<T>`]
    ///
    /// Converts `self` into an [`Option<T>`], consuming `self`, and logs the error, if any.
    fn ok_or_log(self) -> Option<T>
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Ok`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], logging a message provided by the
    /// [`Err`]'s value to a [`tracing::Subscriber`] at an [`ERROR`] level.
    ///
    /// [`ERROR`]: /tracing/0.1/tracing/struct.Level.html#associatedconstant.ERROR
    fn unwrap_or_log(self) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Ok`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], logging the passed message and the
    /// content of the [`Err`] to a [`tracing::Subscriber`] at an [`ERROR`] level.
    ///
    /// [`ERROR`]: /tracing/0.1/tracing/struct.Level.html#associatedconstant.ERROR
    fn expect_or_log(self, msg: &str) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging a message provided by the
    /// [`Ok`]'s value to a [`tracing::Subscriber`] at an [`ERROR`] level.
    ///
    /// [`ERROR`]: /tracing/0.1/tracing/struct.Level.html#associatedconstant.ERROR
    fn unwrap_err_or_log(self) -> E
    where
        T: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging the passed message and the
    /// content of the [`Ok`] to a [`tracing::Subscriber`] at an [`ERROR`] level.
    ///
    /// [`ERROR`]: /tracing/0.1/tracing/struct.Level.html#associatedconstant.ERROR
    fn expect_err_or_log(self, msg: &str) -> E
    where
        T: fmt::Debug;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[inline]
    #[track_caller]
    fn ok_or_log(self) -> Option<T>
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                discarded_with("called `Result::ok_or_log` on an `Err` value", &e);
                None
            }
        }
    }

    #[inline]
    #[track_caller]
    fn unwrap_or_log(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => failed_with("called `Result::unwrap_or_log()` on an `Err` value", &e),
        }
    }

    #[inline]
    #[track_caller]
    fn expect_or_log(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => failed_with(msg, &e),
        }
    }

    #[inline]
    #[track_caller]
    fn unwrap_err_or_log(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => failed_with("called `Result::unwrap_err_or_log()` on an `Ok` value", &t),
            Err(e) => e,
        }
    }

    #[inline]
    #[track_caller]
    fn expect_err_or_log(self, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => failed_with(msg, &t),
            Err(e) => e,
        }
    }
}

//
// Extension trait for Option types.
//

/// Extension trait for Option types.
pub trait OptionExt<T> {
    /// Moves the value `v` out of the `Option<T>` if it is [`Some(v)`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`None`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`None`], logging an error message to a
    /// [`tracing::Subscriber`] at an [`ERROR`] level.
    fn unwrap_or_log(self) -> T;

    /// Unwraps an option, yielding the content of a [`Some`].
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`None`], logging the passed message to a
    /// [`tracing::Subscriber`] at an [`ERROR`] level.
    fn expect_or_log(self, msg: &str) -> T;

    /// Unwraps an option, expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], logging a message derived from the
    /// [`Some`]'s value to a [`tracing::Subscriber`] at an [`ERROR`] level.
    fn unwrap_none_or_log(self)
    where
        T: fmt::Debug;

    /// Unwraps an option, expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], logging the passed message and the
    /// content of the [`Some`] to a [`tracing::Subscriber`] at an [`ERROR`] level.
    fn expect_none_or_log(self, msg: &str)
    where
        T: fmt::Debug;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    #[track_caller]
    fn unwrap_or_log(self) -> T {
        match self {
            Some(val) => val,
            None => failed("called `Option::unwrap_or_log()` on a `None` value"),
        }
    }

    #[inline]
    #[track_caller]
    fn expect_or_log(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => failed(msg),
        }
    }

    #[inline]
    #[track_caller]
    fn unwrap_none_or_log(self)
    where
        T: fmt::Debug,
    {
        if let Some(val) = self {
            failed_with(
                "called `Option::unwrap_none_or_log()` on a `Some` value",
                &val,
            );
        }
    }

    #[inline]
    #[track_caller]
    fn expect_none_or_log(self, msg: &str)
    where
        T: fmt::Debug,
    {
        if let Some(val) = self {
            failed_with(msg, &val);
        }
    }
}

//
// Helper functions.
//

#[inline(never)]
#[cold]
#[track_caller]
fn failed(msg: &str) -> ! {
    #[cfg(feature = "log-location")]
    {
        let location = std::panic::Location::caller();
        tracing::error!(
            unwrap.filepath = location.file(),
            unwrap.lineno = location.line(),
            unwrap.columnno = location.column(),
            "{}",
            msg
        );
    }

    #[cfg(not(feature = "log-location"))]
    tracing::error!("{}", msg);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}", msg)
}

#[inline(never)]
#[cold]
#[track_caller]
fn failed_with(msg: &str, value: &dyn fmt::Debug) -> ! {
    #[cfg(feature = "log-location")]
    {
        let location = std::panic::Location::caller();
        tracing::error!(
            unwrap.filepath = location.file(),
            unwrap.lineno = location.line(),
            unwrap.columnno = location.column(),
            "{}: {:?}",
            msg,
            &value
        );
    }

    #[cfg(not(feature = "log-location"))]
    tracing::error!("{}: {:?}", msg, &value);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}: {:?}", msg, &value);
}

#[inline(never)]
#[cold]
#[track_caller]
fn discarded_with(msg: &str, value: &dyn fmt::Debug) {
    #[cfg(feature = "log-location")]
    {
        let location = std::panic::Location::caller();
        tracing::error!(
            unwrap.filepath = location.file(),
            unwrap.lineno = location.line(),
            unwrap.columnno = location.column(),
            "{}: {:?}",
            msg,
            &value
        );
    }

    #[cfg(not(feature = "log-location"))]
    tracing::error!("{}: {:?}", msg, &value);
}
