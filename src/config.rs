use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub ckb_ws_url: String,
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub mqtt_client_id: String,
    pub concurrent_requests: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, crate::error::Error> {
        Ok(Config {
            ckb_ws_url: std::env::var("CKB_WS_URL")
                .unwrap_or_else(|_| "ws://localhost:8114".to_string()),
            mqtt_host: std::env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_port: std::env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse()
                .map_err(|_| crate::error::Error::Config("Invalid MQTT port".to_string()))?,
            mqtt_client_id: std::env::var("MQTT_CLIENT_ID")
                .unwrap_or_else(|_| "ckb-tx-detail-service".to_string()),
            concurrent_requests: std::env::var("CONCURRENT_REQUESTS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        })
    }
}
