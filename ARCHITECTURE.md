# Persona Signal Architecture

`persona-signal` owns Persona's binary contract between Rust components. It is
the single home for rkyv records that cross process boundaries.

```mermaid
flowchart LR
  "message CLI" -->|"typed request"| "Frame"
  "persona-router" -->|"delivery request"| "Frame"
  "persona-store" -->|"commit reply"| "Frame"
  "persona-system" -->|"focus event"| "Frame"
  "Frame" -->|"rkyv length-prefixed bytes"| "Unix socket"
```

The crate owns:

- the `Frame` envelope;
- 4-byte big-endian length-prefix framing;
- handshake and protocol-version records;
- auth-proof record shapes;
- request and reply enums;
- system-event and delivery-decision records;
- schema-version records for boot-time compatibility checks.

It does not own daemon loops, actor supervision, storage tables, terminal
adapters, NOTA parsing, or policy interpretation. Those belong to the consuming
components.

