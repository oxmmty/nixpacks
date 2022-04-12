use serde::{Deserialize, Serialize};

use super::{
    environment::EnvironmentVariables,
    nix::NixConfig,
    phase::{BuildPhase, InstallPhase, SetupPhase, StartPhase},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildPlan {
    pub version: String,
    pub setup: SetupPhase,
    pub install: InstallPhase,
    pub build: BuildPhase,
    pub start: StartPhase,
    pub variables: EnvironmentVariables,
}
