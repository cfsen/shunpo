use std::path::PathBuf;
use nucleo::Utf32String;

use crate::search::item_source::{
    populate_binaries,
    populate_scripts,
    populate_documents,
};

pub struct SearchItems{
    pub executables: Vec<Executable>,
    pub scripts: Vec<ShellScript>,
    pub documents: Vec<DocumentDirectory>,
}
impl SearchItems {
    pub fn new() -> Self {
        Self {
            executables: populate_binaries(),
            scripts: populate_scripts(),
            documents: populate_documents(),
        }
    }
    pub fn populate(&mut self) -> &mut Self {
        self.executables = populate_binaries();
        self.scripts = populate_scripts();
        self.documents = populate_documents();
        self
    }
    pub fn rebuild(&mut self) -> &mut Self {
        // TODO: check when last populated, skip if recent
        self.populate();
        self
    }
}

#[derive(Clone)]
pub enum SearchResult {
    Executable(Executable),
    ShellScript(ShellScript),
    DocumentDirectory(DocumentDirectory),
}

#[derive(Clone)]
pub struct Executable {
    pub name: Utf32String,
    pub path: PathBuf,
    pub dispatch_with: Dispatcher,
}
impl MatchField for Executable {
    fn get_match_field(&self) -> &Utf32String { &self.name }
}

#[derive(Clone)]
pub enum Dispatcher {
    Hyprctl,
    Shell,
}

#[derive(Clone)]
pub struct ShellScript {
    pub name: Utf32String,
    pub path: PathBuf,
}
impl MatchField for ShellScript {
    fn get_match_field(&self) -> &Utf32String { &self.name }
}

#[derive(Clone)]
pub struct DocumentDirectory {
    name: Utf32String,
    path: PathBuf,
}
impl MatchField for DocumentDirectory {
    fn get_match_field(&self) -> &Utf32String { &self.name }
}

pub trait MatchField {
    fn get_match_field(&self) -> &Utf32String;
}

