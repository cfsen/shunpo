use std::path::PathBuf;

use crate::{config::config::ShunpoConfig, search::{entity_loader::{scan_desktop_executables, scan_path_executables, scan_script_executables}, entity_model::{ExecutableEntity, FileEntity, RipgrepEntity}}};

pub struct EntityRepository {
    pub exec_desktop: Vec<ExecutableEntity>,
    pub executables: Vec<ExecutableEntity>,
    pub scripts: Vec<ExecutableEntity>,
    pub documents: Vec<RipgrepEntity>,

    pub generic_exec_desktop: Vec<FileEntity>,
    pub generic_executables: Vec<FileEntity>,
    pub generic_shell_scripts: Vec<FileEntity>,

    pub config: RepositoryConfig,
}
impl EntityRepository {
    pub fn new(config: RepositoryConfig) -> Self {
        EntityRepository {
            exec_desktop: Vec::new(),
            executables: Vec::new(),
            scripts: Vec::new(),
            documents: Vec::new(),
            generic_exec_desktop: Vec::new(),
            generic_executables: Vec::new(),
            generic_shell_scripts: Vec::new(),
            config,
        }
    }
    pub fn populate(&mut self) -> &mut Self {
        self.exec_desktop.clear();
        self.executables.clear();
        self.scripts.clear();
        self.documents.clear();
        self.generic_executables.clear();

        self.exec_desktop = scan_desktop_executables(self.config.exec_paths.clone());
        self.executables = scan_path_executables();
        self.scripts = scan_script_executables(&self.config.script_paths);
        self.generic_executables = EntityRepository::build_generic_executables(&self.executables);
        self.generic_exec_desktop = EntityRepository::build_generic_executables(&self.exec_desktop);
        self.generic_shell_scripts = EntityRepository::build_generic_executables(&self.scripts);

        self
    }
    pub fn rebuild(self) -> Self {
        EntityRepository::new(self.config)
    }

    //
    // executables
    //

    pub fn get_generic_executables(&self) -> &Vec<FileEntity> {
        &self.generic_executables
    }
    pub fn get_generic_exec_desktop(&self) -> &Vec<FileEntity> {
        &self.generic_exec_desktop
    }

    pub fn get_generic_scripts(&self) -> &Vec<FileEntity> {
        &self.generic_shell_scripts
    }

    pub fn build_generic_executables(entities: &Vec<ExecutableEntity>) -> Vec<FileEntity> {
        entities.iter()
            .cloned()
            .map(|e| FileEntity::Executable(e))
            .collect::<Vec<FileEntity>>()
    }
}

pub struct RepositoryConfig {
    pub exec_paths: Vec<PathBuf>,
    pub script_paths: Vec<PathBuf>,
}
impl RepositoryConfig {
    pub fn from_shunpo_config(config: &ShunpoConfig) -> RepositoryConfig {
        let exec_paths = Self::get_valid_paths(&config.desktop_entries_paths);
        let script_paths = Self::get_valid_paths(&config.script_paths);

        RepositoryConfig {
            exec_paths,
            script_paths,
        }
    }
    fn get_valid_paths(paths: &[String]) -> Vec<PathBuf> {
        paths.iter()
            .map(|p| PathBuf::from(p))
            .filter(|p| p.exists())
            .collect()
    }
}
