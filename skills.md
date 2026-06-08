# skills — signal-persona

This repository is the ordinary Persona working-signal contract. It exposes
manager-to-supervised-component lifecycle traffic: announce, readiness, health,
graceful stop, and the typed `SpawnEnvelope`.

The Persona contract pair is:

- `signal-persona` for ordinary lifecycle traffic;
- `meta-signal-persona` for privileged Persona engine-manager commands.

Do not add privileged policy payloads here. Add those to
`meta-signal-persona`.
