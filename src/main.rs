#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|error| anyhow::anyhow!("logging setup: {error}"))?;
    jet_bridge::cli::execute(&jet_bridge::cli::command().get_matches()).await
}
