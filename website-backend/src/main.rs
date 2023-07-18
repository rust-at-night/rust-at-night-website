use clap::Parser;
use color_eyre::Result;
use website_backend_lib::Options;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize color_eyre.
    color_eyre::install()?;

    // Parse the environment.
    let options = Options::parse();

    // Setup tracing.
    options.tracing_setup();

    // Initialize the service
    match options.make_service().await {
        Ok(()) => Ok(()),
        Err(err) => {
            tracing::error!("{err:#}");
            Err(err)
        }
    }
}
