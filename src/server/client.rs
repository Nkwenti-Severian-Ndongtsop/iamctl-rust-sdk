use crate::server::types::{JsonRpcRequest, JsonRpcResponse};
use crate::utils::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdout, Command};

/// A JSON-RPC client for communicating with a provider process.
/// Useful for testing and for the engine to call providers.
pub struct JsonRpcClient {
    child: Child,
    stdout_reader: BufReader<ChildStdout>,
}

impl JsonRpcClient {
    /// Launches a provider executable and initializes the client.
    pub fn launch(executable_path: &str) -> Result<Self> {
        let mut child = Command::new(executable_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().ok_or_else(|| {
            crate::utils::Error::Internal(
                "Failed to capture stdout of provider process".to_string(),
            )
        })?;

        Ok(Self {
            child,
            stdout_reader: BufReader::new(stdout),
        })
    }

    /// Calls a method on the provider and returns the result.
    pub async fn call<P, R>(&mut self, method: &str, params: P) -> Result<R>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let id = serde_json::Value::Number(1.into());
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: serde_json::to_value(params)?,
            id: Some(id),
        };

        let mut json = serde_json::to_string(&request)?;
        json.push('\n');

        let stdin = self.child.stdin.as_mut().ok_or_else(|| {
            crate::utils::Error::Internal("Failed to access provider stdin".to_string())
        })?;

        stdin.write_all(json.as_bytes()).await?;
        stdin.flush().await?;

        let mut response_line = String::new();
        self.stdout_reader.read_line(&mut response_line).await?;

        let response: JsonRpcResponse = serde_json::from_str(&response_line)?;

        if let Some(error) = response.error {
            return Err(crate::utils::Error::Protocol(format!(
                "JSON-RPC Error {}: {}",
                error.code, error.message
            )));
        }

        let result = response.result.ok_or_else(|| {
            crate::utils::Error::Protocol("Missing result in JSON-RPC response".to_string())
        })?;

        Ok(serde_json::from_value(result)?)
    }
}
