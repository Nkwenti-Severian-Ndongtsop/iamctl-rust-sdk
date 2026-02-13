# Engine & Provider model

The iamctl ecosystem is split into two roles:

- The **iamctl engine**: the CLI/runtime that evaluates your desired state, manages state, and orchestrates operations.
- A **provider**: an executable that implements IAM domain logic (plan/apply/validate/import).

The iamctl Rust SDK helps you build providers by implementing the protocol and runtime plumbing.

## How the engine talks to a provider

At runtime, **iamctl spawns the provider executable as a subprocess**.

Communication happens over **stdin/stdout** using **JSON-RPC**.

```text
[ iamctl engine ] <--- JSON-RPC (stdin/stdout) ---> [ your provider binary ]
                               |
                               v
                      [ iamctl Rust SDK runtime ]
                               |
                               v
                        [ Provider trait impl ]
```

### What this means for you

- You dont build a web server.
- You dont expose a network port.
- You implement the `Provider` trait, and the SDK handles request routing.

## Provider lifecycle (high level)

- **Capabilities/metadata**: the engine asks what the provider supports.
- **Validate (optional)**: the engine can ask the provider to validate specs.
- **Plan**: compute a list of changes between current and desired state.
- **Apply**: execute the planned changes.
- **Import (optional)**: bring existing resources under management.

## Common implementation pattern

- Define your resource spec structs
- Register schemas for each resource type
- Implement provider operations
- Run the SDK server loop
