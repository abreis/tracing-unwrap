use tracing_unwrap::OptionExt;

#[test]
#[tracing_test::traced_test]
#[cfg_attr(not(feature = "log-location"), ignore)]
fn log_location() {
    let _ = std::panic::catch_unwind(|| {
        Option::<()>::None.unwrap_or_log();
        // Note: if you change anything above here, make sure to adjust the asserts below as well
    });

    assert!(logs_contain("unwrap.filepath=\"tests/log-location.rs\""));
    assert!(logs_contain("unwrap.lineno=8"));
    assert!(logs_contain("unwrap.columnno=28"));
}
