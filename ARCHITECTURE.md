# signal-persona — Architecture

`signal-persona` is the **ordinary working-signal contract for Persona** — the
`signal-<component>` half of Persona's contract pair. It is canonical, not a
shim.

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
  Its ordinary lifecycle surface is exposed here while the generated schema
  source moves into this repository.

`meta-signal-persona` exists and carries the privileged policy surface. This
repo is the ordinary side of the pair.

## Pending schema-engine upgrade

Persona's wire contract migrates to a schema-language source consumed by the
schema-derived emission stack (`schema-next` / `schema-rust-next`), like the
spirit pilot — the contract's Rust is regenerated from its `.schema`, not
hand-written. This crate's cutover follows the persona daemon's.
