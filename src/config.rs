use std::env;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "[::1]:50051".to_string());
        let window_width = env::var("WINDOW_WIDTH")
            .unwrap_or_else(|_| "1980".to_string())
            .parse::<u32>()
            .unwrap_or(1980);
        let window_height = env::var("WINDOW_HEIGHT")
            .unwrap_or_else(|_| "1080".to_string())
            .parse::<u32>()
            .unwrap_or(1080);

        Ok(Config {
            address,
            window_width,
            window_height,
        })
    }
}
