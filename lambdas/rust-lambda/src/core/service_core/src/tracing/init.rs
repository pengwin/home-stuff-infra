pub fn init() {
    tracing_subscriber::fmt()
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();
}
