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

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** Persona's wire contract — what survives of it after the retirement carve-out — converts to a single `persona/persona.schema` file consumed by the brilliant macro library (`primary-ezqx.1`). The macro emits wire types, ShortHeader projection, dispatcher, VersionProjection, and storage descriptors from one source.

**Sequence:** Spirit is the MVP pilot landing first via `primary-ezqx.1`; this contract's schema cutover follows the persona daemon's schema cutover. Because this crate is a retired compatibility shim, the cutover may simply remove the remaining re-exports rather than re-emit them through the macro pipeline; the substantive Persona wire surface is now `owner-signal-persona` + `signal-engine-management` (each gets its own per-component cutover bead). Post-/318 the AttemptHandover verb has been shed from the Persona surface; the upgrade triad owns that vocabulary.

**Per-component concerns:**
- This crate is a retired compatibility shim (per the §"Replacement Repositories" table above); the schema-engine cutover for the Persona wire surface lands in `owner-signal-persona` and `signal-engine-management`, not here.
- The retirement timeline may obsolete this crate before its own schema cutover; in that case the bead deletes the crate rather than rewrites it.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
