pub mod types;

pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

use crate::provider::Provider;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

/// JSON-RPC Server that handles communication with the iamctl engine.
pub struct JsonRpcServer<P: Provider> {
    provider: Arc<P>,
}

impl<P: Provider + 'static> JsonRpcServer<P> {
    /// Creates a new JSON-RPC server with the given provider.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Starts the server and listens for requests on stdin.
    pub async fn serve(&self) -> crate::utils::Result<()> {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin).lines();
        let mut stdout = io::stdout();

        while let Some(line) = reader.next_line().await? {
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    let err_resp = JsonRpcResponse::error(
                        serde_json::Value::Null,
                        -32700,
                        format!("Parse error: {}", e),
                    );
                    self.send_response(&mut stdout, err_resp).await?;
                    continue;
                }
            };

            let response = self.handle_request(request).await;
            self.send_response(&mut stdout, response).await?;
        }

        Ok(())
    }

    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.unwrap_or(serde_json::Value::Null);

        match request.method.as_str() {
            "metadata" => {
                JsonRpcResponse::success(id, serde_json::to_value(self.provider.metadata()).unwrap())
            }
            "capabilities" => JsonRpcResponse::success(
                id,
                serde_json::to_value(self.provider.capabilities()).unwrap(),
            ),
            "plan" => match serde_json::from_value(request.params) {
                Ok(params) => match self.provider.plan(params).await {
                    Ok(res) => JsonRpcResponse::success(id, serde_json::to_value(res).unwrap()),
                    Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            },
            "apply" => match serde_json::from_value(request.params) {
                Ok(params) => match self.provider.apply(params).await {
                    Ok(res) => JsonRpcResponse::success(id, serde_json::to_value(res).unwrap()),
                    Err(e) => JsonRpcResponse::error(id, -32000, e.to_string()),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            },
            _ => {
                JsonRpcResponse::error(id, -32601, format!("Method not found: {}", request.method))
            }
        }
    }

    async fn send_response(
        &self,
        stdout: &mut io::Stdout,
        response: JsonRpcResponse,
    ) -> crate::utils::Result<()> {
        let mut json = serde_json::to_string(&response)?;
        json.push('\n');
        stdout.write_all(json.as_bytes()).await?;
        stdout.flush().await?;
        Ok(())
    }
}
