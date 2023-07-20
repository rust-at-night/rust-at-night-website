use crate::Service;
use camino::Utf8PathBuf;
use clap::{Args, Parser};
use color_eyre::{eyre::Context, Result};
use std::net::SocketAddr;
use tracing::Subscriber;
use tracing_subscriber::{prelude::*, registry::LookupSpan, EnvFilter, Layer};
use url::Url;

/// Aws Options
#[derive(Clone, Debug, Args)]
#[non_exhaustive]
pub struct AwsOptions {
    // TODO: Environment variables about Aws when the infra is ready.
}

/// Service Options
#[derive(Clone, Debug, Parser)]
#[non_exhaustive]
pub struct Options {
    /// Socket address to listen on
    #[clap(short, long, env = "ADDR", default_value = "0.0.0.0:8081")]
    pub addr: SocketAddr,

    /// Url of this service.
    #[clap(long, env = "ROOT_URL", default_value = "http://localhost:8081")]
    pub root_url: Url,

    /// Url of this service.
    #[clap(long, env = "STATIC_DIR", default_value = "../dist")]
    pub static_dir: Utf8PathBuf,

    #[clap(long, env = "DATABASE_URL", default_value = "sqlite::memory:")]
    pub database_url: String,

    #[clap(flatten)]
    pub aws: AwsOptions,

    /// Jaeger collector URL.
    #[cfg(feature = "tracing-jaeger")]
    #[clap(long, env, default_value = "http://localhost:14268/api/traces")]
    pub jaeger_collector: Url,
}

impl Options {
    /// Initializes a service from the given options.
    pub async fn make_service(&self) -> Result<()> {
        let state = Service::from_options(self)
            .await
            .context("Making the service considering given options..")?;

        state.run().await.context("Running service")
    }

    /// Sets up tracing layers.
    pub fn tracing_setup(&self) {
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .expect("Unable to setup tracing filter layer.");

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(Self::tracing_layers())
            .init();
    }

    fn tracing_layers<S: Subscriber + Send + Sync + for<'span> LookupSpan<'span>>(
    ) -> Vec<Box<dyn Layer<S> + Send + Sync + 'static>> {
        #[allow(unused_mut)]
        let mut layers = vec![
            tracing_subscriber::fmt::layer().with_target(false).boxed(),
            // More layers here..
        ];

        #[cfg(feature = "tracing-jaeger")]
        layers.push(self.tracing_layer_jaeger());

        layers
    }

    /// Jaeger tracing layer.
    ///
    /// Only enabled when the feature is set.
    #[cfg(feature = "tracing-jaeger")]
    fn tracing_layer_jaeger<S: Subscriber + Send + Sync + for<'span> LookupSpan<'span>>(
        &self,
    ) -> Box<dyn Layer<S> + Send + Sync + 'static> {
        let tracer = opentelemetry_jaeger::new_collector_pipeline()
            .with_service_name(env!("CARGO_PKG_NAME"))
            .with_endpoint(self.jaeger_collector.as_str())
            .with_reqwest()
            .install_batch(opentelemetry::runtime::Tokio)
            .unwrap();
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        opentelemetry.boxed()
    }
}
