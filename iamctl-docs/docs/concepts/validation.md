# Validation

Validation protects users by catching mistakes early.

The SDK provides:

- **Schema validation** (JSON Schema)
- Optional **custom validation** in `Provider::validate`

## Schema validation

Use `schemars` to derive JSON Schemas from Rust structs.

Then register the schema with the server:

```rust
let mut server = JsonRpcServer::new(MyProvider);
server.register_type_schema::<UserSpec>("user");
```

Incoming requests are validated against the registered schema.

## Custom validation

Use custom validation for things JSON Schema cannot express easily:

- Cross-field constraints
- Remote checks (e.g., unique name)
- Policy rules

Return `ValidateResponse` with:

- `valid: bool`
- `errors: Vec<String>`

## Recommended validation layers

- **Schema**: shape/type checks
- **Custom validate**: business rules
- **Apply-time checks**: enforce safety during execution
