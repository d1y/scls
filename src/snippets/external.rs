use anyhow::Result;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ExternalSnippets {
    pub sources: Vec<SnippetSource>,
}

#[derive(Debug, Deserialize)]
pub struct SnippetSource {
    pub name: Option<String>,
    pub git: String,
    pub paths: Vec<SourcePath>,
}

#[derive(Debug, Deserialize)]
pub struct SourcePath {
    pub scope: Option<Vec<String>>,
    pub path: String,
}

impl SnippetSource {
    pub fn destination_path(&self) -> Result<std::path::PathBuf> {
        match &self.name {
            Some(name) => return Ok(std::path::PathBuf::from_str(&name)?),
            None => todo!(),
        }
    }
}
