use nucleo::Utf32String;
use serde::{Deserialize, Serialize};
use core::fmt;
use std::{fmt::Display, path::PathBuf, process::Command};

use crate::{config::config::ShunpoConfig, search::entity_model::{Dispatcher, Export, FileEntity, LauncherEntity, RipgrepEntity}};

pub enum RipgrepError {
    WIP,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgMatch {
    #[serde(rename = "type")]
    kind: String,
    data: RgMatchData,
}
impl fmt::Display for RgMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:\n{}",
            self.kind,
            self.data)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgMatchData {
    path: RgFieldData,
    lines: RgFieldData,
    line_number: i32,
    absolute_offset: i32,
    submatches: Vec<RgSubMatch>,
}
impl fmt::Display for RgMatchData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  path:    {}", self.path.as_text().unwrap_or("<bytes>"))?;
        writeln!(f, "  line:    {}", self.line_number)?;
        writeln!(f, "  offset:  {}", self.absolute_offset)?;
        writeln!(f, "  content: {}", self.lines.as_text().unwrap_or("<bytes>").trim())?;
        for submatch in &self.submatches {
            writeln!(f, "  match:   {}", submatch)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum RgFieldData {
    Text { text: String },
    Bytes { bytes: Vec<u8> },
}
impl RgFieldData {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            RgFieldData::Text { text } => Some(text),
            RgFieldData::Bytes { .. } => None,
        }
    }
    pub fn as_utf32(&self) -> Utf32String {
        match self {
            RgFieldData::Text { text } => Utf32String::from(text.to_string()),
            RgFieldData::Bytes { bytes } => Utf32String::from(
                String::from_utf8_lossy(bytes).into_owned()
            ),
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            RgFieldData::Text { text } => text.to_string(),
            RgFieldData::Bytes { bytes } => String::from_utf8_lossy(bytes).into_owned(),
        }
    }
}
impl Display for RgFieldData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RgFieldData::Text { text } => write!(f, "{}", text),
            RgFieldData::Bytes { bytes } => write!(f, "<bytes({})>", bytes.len()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RgSubMatch {
    #[serde(rename = "match")]
    result: RgFieldData,
    start: i32,
    end: i32,
}
impl Display for RgSubMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {} | end: {} | match: {}", 
            self.start,
            self.end,
            self.result,
        )
    }
}


fn call_rg(term: &str, path: &str) -> Result<String, RipgrepError> {
    let output = Command::new("rg")
        .arg(term)
        .arg(path)
        .arg("-i")
        .arg("--max-depth")
        .arg("1")
        .arg("--json")
        .output()
        .map_err(|_| RipgrepError::WIP)?;

    String::from_utf8(output.stdout)
        .map_err(|_| RipgrepError::WIP)
}

fn call_rg_serialize(term: &str, path: &str) -> Result<Vec<RgMatch>, RipgrepError> {
    let call_stdout = call_rg(term, path)?;

    let mut results = vec![];
    for line in call_stdout.split("\n") {
        match serde_json::from_str::<RgMatch>(&line) {
            Ok(value) => { results.push(value); },
            Err(_) => { },
        }
    }

    Ok(results)
}

pub fn rg_lookup(
    term: &str,
    config: &ShunpoConfig
) -> Result<Vec<LauncherEntity>, RipgrepError> {
    let mut results = vec![];
    if term == " " { return Ok(results); }

    for rg_path in &config.ripgrep_paths {
        let path_results = call_rg_serialize(term, &rg_path)?;
        for file in path_results {
            let rge = FileEntity::Ripgrep(
                RipgrepEntity {
                    dispatcher: Dispatcher::Hyprctl, // TODO: update to nvim dispatcher
                    match_name: file.data.path.as_utf32(),
                    match_rank: None,
                    path: PathBuf::from(file.data.path.as_string()),
                    ui_name: file.data.path.as_string(),
                    line: file.data.line_number,
                }
            );
            results.push(rge.into_launcher_entity());
        }
    }

    Ok(results)
}
