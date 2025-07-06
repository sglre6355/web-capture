use std::{sync::Arc, time::Duration};

use headless_chrome::{
    Browser, LaunchOptionsBuilder, protocol::cdp::Page::CaptureScreenshotFormatOption,
};

use crate::{
    config::Config,
    error::AppError,
    web_capture::{Interaction, InteractionType},
};

pub struct BrowserService {
    browser: Arc<Browser>,
}

impl BrowserService {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let browser = Arc::new(
            Browser::new(
                LaunchOptionsBuilder::default()
                    .headless(true)
                    .window_size(Some((config.window_width, config.window_height)))
                    .idle_browser_timeout(Duration::MAX)
                    .sandbox(false)
                    .build()
                    .map_err(|e| AppError::Browser(e.to_string()))?,
            )
            .map_err(|e| AppError::Browser(e.to_string()))?,
        );

        Ok(BrowserService { browser })
    }

    pub async fn capture_screenshot(
        &self,
        url: &str,
        element_selector: &str,
        format: CaptureScreenshotFormatOption,
        interactions: &[Interaction],
    ) -> Result<Vec<u8>, AppError> {
        let tab = self
            .browser
            .new_tab()
            .map_err(|e| AppError::Browser(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| AppError::Navigation(e.to_string()))?;

        // Execute all interactions before capturing
        for interaction in interactions {
            self.execute_interaction(&tab, interaction).await?;
        }

        let element = tab
            .wait_for_element(element_selector)
            .map_err(|e| AppError::ElementNotFound(e.to_string()))?;

        let box_model = element
            .get_box_model()
            .map_err(|e| AppError::ElementNotFound(e.to_string()))?;

        let image_data = tab
            .capture_screenshot(format, None, Some(box_model.border_viewport()), true)
            .map_err(|e| AppError::Screenshot(e.to_string()))?;

        Ok(image_data)
    }

    async fn execute_interaction(
        &self,
        tab: &headless_chrome::Tab,
        interaction: &Interaction,
    ) -> Result<(), AppError> {
        match interaction.r#type() {
            InteractionType::Click => {
                let element = tab
                    .wait_for_element(&interaction.selector)
                    .map_err(|e| AppError::ElementNotFound(e.to_string()))?;
                element
                    .click()
                    .map_err(|e| AppError::Interaction(e.to_string()))?;
            }
            InteractionType::Type => {
                let element = tab
                    .wait_for_element(&interaction.selector)
                    .map_err(|e| AppError::ElementNotFound(e.to_string()))?;
                element
                    .type_into(&interaction.value)
                    .map_err(|e| AppError::Interaction(e.to_string()))?;
            }
            InteractionType::Wait => {
                if !interaction.selector.is_empty() {
                    // Wait for element to appear
                    tab.wait_for_element(&interaction.selector)
                        .map_err(|e| AppError::ElementNotFound(e.to_string()))?;
                } else if interaction.wait_ms > 0 {
                    // Fixed time wait
                    tokio::time::sleep(Duration::from_millis(interaction.wait_ms as u64)).await;
                }
            }
            InteractionType::Scroll => {
                let element = tab
                    .wait_for_element(&interaction.selector)
                    .map_err(|e| AppError::ElementNotFound(e.to_string()))?;
                element
                    .scroll_into_view()
                    .map_err(|e| AppError::Interaction(e.to_string()))?;
            }
            InteractionType::Hover => {
                let element = tab
                    .wait_for_element(&interaction.selector)
                    .map_err(|e| AppError::ElementNotFound(e.to_string()))?;
                element
                    .move_mouse_over()
                    .map_err(|e| AppError::Interaction(e.to_string()))?;
            }
            InteractionType::Unspecified => {
                return Err(AppError::Interaction(
                    "Unspecified interaction type".to_string(),
                ));
            }
        }
        Ok(())
    }
}
