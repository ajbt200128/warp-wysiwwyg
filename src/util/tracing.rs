use tracing_subscriber::fmt::format::FmtSpan;

pub(crate) fn start_tracing() {
    // Filter traces based on the RUST_LOG env var, or, if it's not set,
    // default to show the output of the example.
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
    // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
    // Record an event when each span closes. This can be used to time our
    // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();
}
