# State management

State is the engines memory of previously applied resources.

The Rust SDK includes a state backend abstraction and a local file backend.

## Why state matters

State enables:

- Plan diffs (compare desired vs last-known applied)
- Idempotent operations
- Drift detection (depending on provider design)

## Local file backend

The local backend stores state on disk with:

- Restricted permissions
- Locking to prevent concurrent writes
- Basic migration/versioning support

## Concurrency and locking

The SDK uses file locking so two `iamctl` runs cannot corrupt the same state.

Provider authors usually dont need to implement locking themselves  it is handled by the backend.

## Best practices

- Keep state minimal (store stable identifiers and essential outputs)
- Avoid secrets in plaintext; prefer references/ids
- Treat state as an internal implementation detail of the engine
