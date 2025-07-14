use tokio::signal::unix::{SignalKind, signal};
use tonic::transport::Server;
use tracing::info;

mod browser;
mod config;
mod error;
mod service;

use config::Config;
use error::AppError;
use service::WebCaptureService;
use web_capture::web_capture_service_server::WebCaptureServiceServer;

pub mod web_capture {
    use headless_chrome::protocol::cdp::{
        Network::CookieParam, Page::CaptureScreenshotFormatOption,
    };

    tonic::include_proto!("web_capture.v1");

    impl From<Cookie> for CookieParam {
        fn from(proto_format: Cookie) -> Self {
            CookieParam {
                name: proto_format.name,
                value: proto_format.value,
                url: None,
                domain: Some(proto_format.domain),
                path: proto_format.path,
                secure: proto_format.secure,
                http_only: proto_format.http_only,
                same_site: None,
                expires: proto_format.expires.map(|i| i as f64),
                priority: None,
                same_party: None,
                source_scheme: None,
                source_port: None,
                partition_key: None,
            }
        }
    }

    impl From<ImageFormat> for CaptureScreenshotFormatOption {
        fn from(proto_format: ImageFormat) -> Self {
            match proto_format {
                ImageFormat::Png => CaptureScreenshotFormatOption::Png,
                ImageFormat::Jpeg => CaptureScreenshotFormatOption::Jpeg,
                ImageFormat::Webp => CaptureScreenshotFormatOption::Webp,
                ImageFormat::Unspecified => CaptureScreenshotFormatOption::Png,
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    let address = config.address.parse()?;

    let service = WebCaptureService::new(config)?;

    info!("Serving gRPC endpoint at {address}");

    let server = Server::builder()
        .add_service(WebCaptureServiceServer::new(service))
        .serve(address);

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select!(
        result = server => {
            result?
        },
        _ = sigint.recv() => {
            info!("Received SIGINT, terminating...");
        },
        _ = sigterm.recv() => {
            info!("Received SIGTERM, terminating...");
        }
    );

    Ok(())
}
