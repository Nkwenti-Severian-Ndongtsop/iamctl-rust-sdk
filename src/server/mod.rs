pub mod client;
pub mod handler;
pub mod types;

#[cfg(test)]
mod handler_tests;

pub use client::JsonRpcClient;
pub use handler::RequestHandler;
pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

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

    /// Registers a JSON schema for a resource type by deriving it from a Rust type.
    pub fn register_type_schema<T: schemars::JsonSchema>(&mut self, resource_type: &str) {
        self.handler.register_type_schema::<T>(resource_type);
    }

    /// Registers a raw JSON schema for a resource type.
    pub fn register_schema(&mut self, resource_type: &str, schema: serde_json::Value) {
        self.handler.register_schema(resource_type, schema);
    }

    /// Alias for serve() to provide a more standard 'run' method.
    pub async fn run(&self) -> crate::utils::Result<()> {
        self.serve().await
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
                        format!("Parse error: {e}"),
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
