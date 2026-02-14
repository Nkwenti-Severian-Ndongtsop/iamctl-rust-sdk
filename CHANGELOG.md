# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [0.1.2] - 2026-02-14

### Added

- GitHub provider source helpers:
  - `GithubProviderSource::parse` for `github:OWNER/REPO` and `github:OWNER/REPO//subdir`.
  - `derive_github_release_tag` for per-provider tags (`<provider>-v<version>`).
- `decode_spec<T>(&Resource) -> Result<T>` helper for decoding a resource spec into typed structs with resource context in errors.
- Unit tests covering provider source parsing, release tag derivation, and spec decoding.

## [0.1.1] - 2026-02-13

### Changed

- Set the crate documentation URL to GitHub Pages.

## [0.1.0] - 2026-02-13

### Added

- JSON-RPC 2.0 server implementation for provider communication over stdin/stdout.
- Request routing layer (`RequestHandler`) for `metadata`, `capabilities`, `plan`, `apply`, `validate`, and `import` methods.
- Provider SDK surface:
  - `Provider` trait.
  - `ProviderMetadata` and `ProviderCapabilities`.
  - `PlanRequest/PlanResponse`, `ApplyRequest/ApplyResponse`, `ValidateRequest/ValidateResponse`, `ImportRequest/ImportResponse`.
- Core types:
  - `ResourceAddress`, `Resource`, `Change`, `ChangeType`.
- Schema validation framework:
  - `JsonSchemaValidator` with support for registering schemas per resource type.
  - Composite validator (`CompositeValidator`).
  - Validation results and error structures.
- State management:
  - `State` type.
  - File-based backend (`FileBackend`) with locking support.
  - Unix-specific file permission hardening (0600) on saved state.
- Logging utilities (`init_logging`, JSON/non-JSON output modes).
- Example provider showing JSON-RPC server usage and schema registration (`examples/basic-provider`).
- Test suite across core types, state backend behavior, request handling, and validation.

### Changed

- JSON-RPC example startup message now writes to stderr to avoid corrupting the stdout transport.
- Validation now propagates serialization failures instead of silently validating `null`.
- Tracing subscriber initialization now uses a non-panicking init path to avoid test/consumer double-init panics.
- GitHub workflows modernized:
  - Release flow improved (tag/version verification, maintained Rust toolchain action, GitHub Release creation, crates.io publishing).
  - Security workflow updated to maintained actions and improved permissions handling.

### Fixed

- Async correctness: moved blocking Unix state file writes off the Tokio worker thread.
- CI/workflow robustness around outdated dependency reporting and permissions for issue creation.

[0.1.0]: https://github.com/Nkwenti-Severian-Ndongtsop/iamctl-rust-sdk/releases/tag/v0.1.0
