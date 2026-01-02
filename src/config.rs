#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_host: String,
    pub app_port: String,
    pub asset_dir: String,
}

impl Config {
    pub fn init() -> Config {
        let app_name = std::env::var("APP_NAME").unwrap_or("mock-s3-rs".to_string());
        let app_host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
        let app_port = std::env::var("APP_PORT").unwrap_or("8080".to_string());
        let asset_dir = std::env::var("ASSET_DIR").unwrap_or("assets".to_string());
        Config {
            app_name,
            app_host,
            app_port,
            asset_dir,
        }
    }
}
