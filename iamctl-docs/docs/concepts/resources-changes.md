# Resources and changes

Providers operate on **resources** and produce **changes**.

## Resource

A `Resource` represents a single managed object (for example: a user, group, role, policy binding).

A resource usually contains:

- An **address**: stable identifier used by iamctl to track it
- A **spec**: the desired configuration (often derived from YAML/JSON)
- Optional **metadata**: provider-specific fields

## Desired vs current

iamctl always works with two views:

- **Desired state**: what the user wants (from configuration)
- **Current state**: what exists (from state + provider read/import)

The provider compares these views to create a plan.

## Change

A `Change` describes a single action:

- Create
- Update
- Delete
- No-op

Each change contains the `before` and `after` resource payloads, plus a `ChangeType`.

## Planning strategy

Good planning rules:

- Prefer deterministic output
- Keep diffs minimal (only change what must change)
- Separate computed fields (fields only known after apply)

## Applying strategy

Apply should:

- Execute changes in a safe order
- Return which addresses succeeded/failed
- Emit structured logs so users understand what happened
