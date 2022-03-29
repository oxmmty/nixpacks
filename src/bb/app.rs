use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct App {
    pub source: PathBuf,
    pub paths: Vec<PathBuf>,
}

impl App {
    pub fn new(source: PathBuf) -> Result<App> {
        let dir = fs::read_dir(source.clone()).context("Failed to read app source directory")?;
        let paths: Vec<PathBuf> = dir.map(|path| path.unwrap().path()).collect();

        Ok(App { source, paths })
    }

    pub fn includes_file(&self, name: &str) -> bool {
        for path in &self.paths {
            if path.file_name().unwrap() == name {
                return true;
            }
        }

        false
    }

    pub fn read_file(&self, name: &str) -> Result<String> {
        let name = self.source.join(name);
        let contents = fs::read_to_string(name)?;
        return Ok(contents);
    }
}
