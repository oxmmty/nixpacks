use anyhow::Result;

use crate::{providers::Pkg, nixpacks::app::App, python::pyproject};

use super::Provider;

pub struct PythonProvider {}
impl Provider for PythonProvider {
    fn name(&self) -> &str {
        "python"
    }

    fn detect(&self, app: &crate::nixpacks::app::App) -> anyhow::Result<bool> {
        Ok(app.includes_file("main.py") || app.includes_file("requirements.txt") || app.includes_file("setup.py") || app.includes_file("setup.cfg"))
    }

    fn pkgs(&self, _app: &crate::nixpacks::app::App) -> Vec<super::Pkg> {
        vec![
            Pkg::new("pkgs.python38")
        ]
    }

    fn install_cmd(&self, app: &App) -> Result<Option<String>> {
        if app.includes_file("requirements.txt") {
            return Ok(Some("python -m ensurepip && python -m pip install -r requirements.txt".to_string()))
        } else if app.includes_file("setup.py") || app.includes_file("setup.cfg") {
            return Ok(Some("python -m ensurepip && python -m pip install .".to_string()))
        }
        Ok(None)
    }

    fn suggested_build_cmd(&self, _app: &App) -> Result<Option<String>> {
        Ok(None)
    }

    fn suggested_start_command(&self, app: &App) -> Result<Option<String>> {
        if app.includes_file("pyproject.toml") {
            if matches!(pyproject::parse(app), Ok(meta)) && matches!(meta.entry_point, Some(entry_point)) {
                
            }
        }
        // falls through
        if app.includes_file("main.py") {
            return Ok(Some("python main.py".to_string()))
        }
        Ok(None)
    }
}