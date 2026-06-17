# signal-persona

The **ordinary working-signal contract for Persona** — the `signal-<component>`
half of Persona's contract pair. Every component has exactly two contracts:
`signal-<component>` (ordinary working signal) and `meta-signal-<component>`
(meta policy signal). For Persona that pair is **`signal-persona`** (this repo,
ordinary) and **`meta-signal-persona`** (meta / privileged policy).

This carries the ordinary Persona engine-management lifecycle traffic
(announce, readiness query, health query, graceful stop, the typed
`SpawnEnvelope`). The privileged policy surface (engine launch, retirement,
component start/stop) belongs in `meta-signal-persona`.

The contract is generated from `schema/lib.schema`; refresh the checked-in
artifact with `SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS=1 cargo build
--all-features`.

> Note: `signal-persona` is **not** a retired shim. The earlier framing that
> split Persona into `owner-signal-persona` + `signal-engine-management` was a
> deviation from the two-contract invariant. The privileged surface is now
> `meta-signal-persona`; this crate exposes the ordinary lifecycle surface.
> Per psyche 2026-06-07 (Spirit `n0ss`).
