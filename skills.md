# signal-persona skill

This repository is a contract crate. Add only shared signal records, their
encoding methods, and round-trip tests.

Rules for work here:

- Keep runtime code out: no daemons, actors, tokio loops, redb stores, terminal
  adapters, or CLI parsing.
- Add a record here only when two or more Persona components need to signal the
  same typed value.
- Persona owns record kinds and query payloads, not protocol verbs. The only
  operation verbs are the twelve `signal-core` Sema verbs.
- Do not add schema fields for infrastructure-minted values: record identity,
  sender principal, or commit time.
- Keep request/reply/event enums closed and typed. Do not add `Unknown` to
  avoid coordinated schema work.
- Tests should encode and decode real records through `Frame`; do not prove
  behavior by grepping IDs or string prefixes.
- Human-facing NOTA belongs in boundary crates such as `persona-message`, not in
  this crate.
