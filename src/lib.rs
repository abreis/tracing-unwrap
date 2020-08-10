use std::fmt;

//
// Extension trait for Result types.
//

/// Extension trait for Result types.
pub trait ResultExt<T, E> {
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
    // #[track_caller]
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
    // #[track_caller]
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
    // #[track_caller]
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
    // #[track_caller]
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
    fn unwrap_or_log(self) -> T {
        match self {
            Some(val) => val,
            None => failed("called `Option::unwrap_or_log()` on a `None` value"),
        }
    }

    #[inline]
    // #[track_caller]
    fn expect_or_log(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => failed(msg),
        }
    }

    #[inline]
    // #[track_caller]
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
    // #[track_caller]
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
// #[track_caller]
fn failed(msg: &str) -> ! {
    tracing::error!("{}", msg);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}", msg)
}

#[inline(never)]
#[cold]
// #[track_caller]
fn failed_with(msg: &str, value: &dyn fmt::Debug) -> ! {
    tracing::error!("{}: {:?}", msg, &value);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}: {:?}", msg, &value);
}
