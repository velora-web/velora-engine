fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .compact()
        .init();

    tracing::info!("cinder_browser minimal example running");
}

