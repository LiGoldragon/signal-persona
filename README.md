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

> Note: `signal-persona` is **not** a retired shim. The earlier framing that
> split Persona into `owner-signal-persona` + `signal-engine-management` was a
> deviation from the two-contract invariant. `owner-signal-persona` is the
> deprecated OwnerSignal form (OwnerSignal → MetaSignal); its surface folds into
> `meta-signal-persona`, and `signal-engine-management` folds into this crate.
> Per psyche 2026-06-07 (Spirit `n0ss`).
