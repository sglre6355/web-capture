use chrono::Utc;
use tonic::{Request, Response, Status};
use tracing::info;

use crate::{
    browser::BrowserService,
    config::Config,
    error::AppError,
    web_capture::{
        CaptureElementRequest, CaptureElementResponse,
        web_capture_service_server::WebCaptureService as WebCaptureServiceTrait,
    },
};

pub struct WebCaptureService {
    browser_service: BrowserService,
}

impl WebCaptureService {
    pub fn new(config: Config) -> Result<Self, AppError> {
        let browser_service = BrowserService::new(&config)?;

        Ok(WebCaptureService { browser_service })
    }
}

#[tonic::async_trait]
impl WebCaptureServiceTrait for WebCaptureService {
    async fn capture_element(
        &self,
        request: Request<CaptureElementRequest>,
    ) -> Result<Response<CaptureElementResponse>, Status> {
        info!("Element capture request received");

        let req = request.into_inner();
        let url = &req.url;
        let element_selector = &req.element_selector;
        let image_format = req.image_format();
        let cookies = req
            .cookies
            .into_iter()
            .map(|cookie| cookie.into())
            .collect();

        let image_data = self
            .browser_service
            .capture_screenshot(
                url,
                element_selector,
                image_format.into(),
                &req.interactions,
                cookies,
            )
            .await
            .map_err(Status::from)?;

        info!("Successfully captured element screenshot");

        let response = CaptureElementResponse {
            image_format: image_format.into(),
            timestamp: Utc::now().timestamp(),
            image_data,
        };

        Ok(Response::new(response))
    }
}
