use schema_rust::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-persona",
        "0.2.0",
        "SIGNAL_PERSONA_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}
