# signal-persona

Engine-manager contract for `persona`.

This crate defines the typed payload records for clients talking to the
top-level `persona` engine manager over `signal-core` frames. The manager
surface reports engine status, component health, and engine-visible
projections; component-to-component behavior uses the relation-specific
`signal-persona-*` contracts.

Human-facing Nexus or NOTA text remains outside this repository.
