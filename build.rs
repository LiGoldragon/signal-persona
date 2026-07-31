use protos::WireContractFamily;
use schema_rust::build::{ContractCrateBuild, CrateName, SchemaVersion, UpdateEnvironmentVariable};

fn main() {
    ContractCrateBuild::from_environment(
        CrateName::new("signal-persona"),
        SchemaVersion::new("0.2.0"),
        UpdateEnvironmentVariable::new("SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS"),
        WireContractFamily::SignalSpirit,
    )
    .expect_fresh();
}
