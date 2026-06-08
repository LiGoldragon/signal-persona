# INTENT — signal-persona

*The ordinary Persona working-signal contract. Companion to
`ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-persona` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
The privileged Persona policy surface stays in `meta-signal-persona`.

## Why this repo exists

`signal-persona` is the ordinary side of the Persona contract pair. It carries
the manager-to-supervised-component lifecycle traffic that makes a process a
Persona component: announce, readiness, health, graceful stop, and the typed
`SpawnEnvelope`.

Every component has exactly two contracts: `signal-<component>` for ordinary
working traffic and `meta-signal-<component>` for privileged policy. For
Persona, the pair is `signal-persona` and `meta-signal-persona`.

## Channel shape

The ordinary lifecycle surface currently reuses the typed records from
`signal-engine-management` while that off-pattern crate is retired into this
repository. The public surface is exposed from `signal-persona` as:

- `Operation` / `OperationKind` / `Reply` for lifecycle requests and replies;
- `Presence`, readiness, health, and stop payloads;
- `SpawnEnvelope`, `PeerSocket`, `WirePath`, and `SocketMode` for supervised
  component startup.

## Constraints

- Privileged Persona policy commands do not live here. They live in
  `meta-signal-persona`.
- Request payloads do not carry caller identity, timestamps, or authorization
  proof; those facts are infrastructure-owned.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only typed wire vocabulary and round-trip witnesses: no
  daemon actors, persistence, process spawning, socket policy, or CLI parsing.

## See also

- `ARCHITECTURE.md` — boundary and current implementation status.
- `../meta-signal-persona/INTENT.md` — privileged Persona policy surface.
- `../signal-engine-management/INTENT.md` — source contract being folded into
  this ordinary surface.
- `../signal-persona-origin/ARCHITECTURE.md` — shared origin/identity
  vocabulary.
