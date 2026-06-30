# signal-persona — Architecture

`signal-persona` is the **ordinary working-signal contract for Persona** — the
`signal-<component>` half of Persona's contract pair. It is canonical,
schema-derived, and not a shim.

It carries the manager-to-supervised-component lifecycle traffic that makes a
process a Persona component: announce, readiness, health, graceful stop, and the
typed `SpawnEnvelope`.

## Invariant — two contracts per component

Every component has exactly two contracts: `signal-<component>` (ordinary
working signal) and `meta-signal-<component>` (meta policy signal). There is no
third contract and no `owner-signal-*` channel. For Persona:

| Repository | Role |
|---|---|
| `signal-persona` (this repo) | ordinary working signal — engine-management lifecycle traffic: announce, readiness, health, graceful stop, the typed `SpawnEnvelope` |
| `meta-signal-persona` | meta policy signal — privileged surface: engine launch, retirement, component start/stop, status policy |

Per psyche 2026-06-07 (Spirit `n0ss`).

## Deviations being retired into this pair

The earlier framing split Persona into three crates and labelled this one a
"retired compatibility shim." That was wrong. The deviations:

- `owner-signal-persona` — the deprecated **OwnerSignal** form (OwnerSignal →
  MetaSignal, Spirit `hnpo`). Its privileged surface folds into
  `meta-signal-persona`.
- `signal-engine-management` — an off-pattern name (not `signal-<component>`).
  Its ordinary lifecycle surface is exposed here from this repository's
  `schema/lib.schema` source.

`meta-signal-persona` exists and carries the privileged policy surface. This
repo is the ordinary side of the pair.

## Schema emission

`schema/lib.schema` is the source of truth for the ordinary lifecycle wire
contract. `build.rs` runs `schema-rust`'s wire-contract driver and
freshness-checks the generated artifact in `src/schema/lib.rs`; regenerate with
`SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS=1 cargo build --all-features` after
schema edits.

The crate re-exports the generated surface from `src/lib.rs`. `Input`,
`InputRoute`, and `Output` are the generated roots; `Operation`,
`OperationKind`, and `Reply` remain aliases for the ordinary lifecycle relation.
The spawn envelope uses role-specific path and socket-mode newtypes so repeated
wire primitives do not hide distinct roles.

## Constraints

- Request payloads do not carry caller identity, timestamps, or authorization
  proof; those facts are infrastructure-owned.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only schema-derived typed wire vocabulary and round-trip
  witnesses: no daemon actors, persistence, process spawning, socket policy, or
  CLI parsing.
