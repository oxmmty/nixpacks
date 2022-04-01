use serde::Serialize;

use crate::providers::Pkg;

#[derive(Debug, Serialize)]
pub struct BuildPlan {
    pub version: String,
    pub nixpkgs_archive: String,
    pub pkgs: Vec<Pkg>,
    pub install_cmd: Option<String>,
    pub start_cmd: Option<String>,
    pub build_cmd: Option<String>,
}
