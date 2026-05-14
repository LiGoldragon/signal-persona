# signal-persona skill

This repository is the contract crate for talking to the top-level `persona`
engine manager. Add only manager-surface signal records, their encoding
methods, and round-trip tests.

Rules for work here:

- Keep runtime code out: no daemons, actors, tokio loops, redb stores, terminal
  adapters, or CLI parsing.
- Add a record here only when it belongs to the `persona` engine manager's
  status, control, query, projection, or subscription surface.
- Component-to-component records belong in the relation-specific
  `signal-persona-*` contract repo for that relation.
- The manager contract owns record kinds and query payloads, not protocol
  verbs. The only operation roots are the seven `signal-core`
  `SignalVerb` roots.
- Do not add schema fields for infrastructure-minted values: record identity,
  sender principal, or commit time.
- Keep request/reply/event enums closed and typed. Do not add `Unknown` to
  avoid coordinated schema work.
- Tests should encode and decode real records through `Frame`; do not prove
  behavior by grepping IDs or string prefixes.
- Contract values derive their NOTA text projection here beside their rkyv wire
  shape. Boundary crates decide where that NOTA is accepted or printed; they do
  not mirror these types just to make them readable.
