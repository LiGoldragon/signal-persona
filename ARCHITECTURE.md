# signal-persona — Architecture

`signal-persona` is a retired compatibility shim. It no longer owns an
authority boundary.

## Replacement Repositories

| Repository | Authority Boundary |
|---|---|
| `owner-signal-persona` | owner-only Persona engine-manager commands: launch, retire, start, stop, status query |
| `signal-engine-management` | ordinary manager-to-supervised-component lifecycle traffic: announce, readiness, health, graceful stop, spawn envelope |

New code depends on one of those repositories directly. This crate only
re-exports enough names to keep older consumers compiling while the workspace
is migrated.

## Invariant

The previous two-channel exception is closed. Persona follows the normal triad
shape: owner authority lives in an `owner-signal-*` repo, and ordinary working
traffic lives in a `signal-*` repo.
