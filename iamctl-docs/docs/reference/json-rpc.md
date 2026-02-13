# JSON-RPC protocol (overview)

The iamctl engine communicates with providers using **JSON-RPC** over stdin/stdout.

You typically do not need to implement this protocol manually when using the Rust SDK.

## Message types

- Request: `{ jsonrpc, id, method, params }`
- Response: `{ jsonrpc, id, result }` or `{ jsonrpc, id, error }`
- Notifications: `{ jsonrpc, method, params }` (no id)

## Methods

The SDK routes engine calls to your `Provider` implementation.

Typical methods include:

- Capabilities
- Plan
- Apply
- Validate
- Import

## Error handling

Providers should return structured errors.

The SDK will translate errors into JSON-RPC error responses.

## Debugging

Enable logging and capture stdin/stdout traffic during development.
