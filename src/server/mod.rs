pub mod types;
pub mod handler;

pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
pub use handler::RequestHandler;

use crate::provider::Provider;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

/// JSON-RPC Server that handles communication with the iamctl engine.
pub struct JsonRpcServer<P: Provider> {
    handler: RequestHandler<P>,
}

impl<P: Provider + 'static> JsonRpcServer<P> {
    /// Creates a new JSON-RPC server with the given provider.
    pub fn new(provider: P) -> Self {
        Self {
            handler: RequestHandler::new(Arc::new(provider)),
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

            let response = self.handler.handle(request).await;
            self.send_response(&mut stdout, response).await?;
        }

        Ok(())
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
