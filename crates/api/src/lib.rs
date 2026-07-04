use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

pub struct ApiServer;

impl ApiServer {
    pub fn build_router() -> Router {
        Router::new()
            .route("/api/status", get(Self::handle_status))
            .route("/api/workflow", get(Self::handle_workflow))
    }

    async fn handle_status() -> Json<Value> {
        Json(json!({
            "status": "active",
            "assistant": "Friday AI",
            "mode": "Coworker"
        }))
    }

    async fn handle_workflow() -> Json<Value> {
        Json(json!({
            "workflows": [
                { "name": "browser_open", "enabled": true },
                { "name": "desktop_screenshot", "enabled": true }
            ]
        }))
    }

    pub async fn run_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let app = Self::build_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_routes() {
        let _router = ApiServer::build_router();
    }
}
