# Provider Ecosystem Gaps (Work Items)

This document captures items that we implemented manually in the `iamctl-providers` repository (starting with the Keycloak provider) which are expected to become first-class capabilities in `iamctl-rust-sdk` (or in the broader iamctl toolchain).

The goal is to:

- keep provider implementations thin and consistent
- avoid each provider re-implementing distribution/decoding/planning patterns
- enable the iamctl CLI to install and verify providers in a deterministic way

## 1) Provider source + installation contract (GitHub Releases)

### What we do today (manual conventions)

In the providers monorepo we currently use:

- Provider **logical name**: `keycloak`
- Provider **binary name**: `iamctl-provider-keycloak`
- Source syntax (GitHub + subdir):

  - `github:Nkwenti-Severian-Ndongtsop/iamctl-providers//keycloak`

- Version is semver only (clean config):

  - `version: "0.1.0"`

- CLI derives the **release tag**:

  - `<provider>-v<version>` (example: `keycloak-v0.1.0`)

- Release assets (Linux-only for now):

  - `iamctl-provider-<provider>_linux_amd64`
  - `iamctl-provider-<provider>_linux_arm64`
  - `checksums.txt` (sha256)

### Desired SDK/tooling support

A canonical implementation of:

- Source parsing:
  - `github:<owner>/<repo>//<subdir>`
- Version-to-tag derivation:
  - `<provider>-v<version>`
- Asset naming helpers:
  - `(provider, os, arch) -> asset_name`
- Checksum format + verification helpers
- Cache directory conventions (where binaries are installed)

### Candidate API surface (conceptual)

- `ProviderSource` enum / parser
- `ProviderReleaseResolver` for GitHub (tags, assets)
- `ProviderAssetSelector` (linux/amd64, linux/arm64)
- `ChecksumVerifier` (sha256)

> Note: this could live in the SDK or in the CLI. The important part is that the contract is defined in one place and reused.

## 2) Typed spec decoding helper (Value -> T)

### What we do today

Providers often do this repeatedly:

- `serde_json::to_value(spec)`
- `serde_json::from_value::<T>(...)`

and manually attach context.

### Desired SDK support

A helper that:

- decodes `ResourceInstance.spec` into a typed model
- includes helpful context (resource address, resource type)
- returns a consistent error shape

## 3) Planning/diff helpers

### What we do today

Plan logic is simplistic:

- determine `Create` vs `Update` by comparing existence
- no field-level diff
- no computed field derivation

### Desired SDK support

- canonical diff logic between `before` and `after`
- support for:
  - computed fields
  - ignored fields
  - default normalization
  - stable ordering

## 4) Provider test kit (optional)

### What we do today

Providers implement their own integration-test harness patterns:

- readiness probes
- login helpers
- caching (OnceCell)
- error formatting

### Desired SDK support (optional)

A test module providing:

- readiness probes / retry helpers
- consistent error messages

## 5) Release automation templates (docs/scaffolding)

Not necessarily an SDK runtime feature, but it would help to maintain:

- recommended GitHub Actions templates for provider repos
- recommended release asset naming/checksum format

## Tracking

- Once these items exist upstream, providers should be refactored to:
  - remove ad-hoc conventions
  - call the shared helpers
  - keep provider-specific logic focused on the target API (Keycloak/AWS/etc.)
