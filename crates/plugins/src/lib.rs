use serde_json::Value;
use std::collections::HashMap;

pub struct PluginContext {
    pub env_vars: HashMap<String, String>,
}

#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> &str;

    async fn execute(&self, action: &str, args: Value, ctx: &PluginContext) -> Result<Value, Box<dyn std::error::Error + Send + Sync>>;
}
