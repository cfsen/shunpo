use std::path::PathBuf;

use crate::search::{entity_loader::scan_path_executables, entity_model::{ExecutableEntity, FileEntity, RipgrepEntity}};

pub struct EntityRepository {
    pub executables: Vec<ExecutableEntity>,
    pub documents: Vec<RipgrepEntity>,

    pub generic_executables: Vec<FileEntity>,
    // pub generic_documents: Vec<FileEntity>,

    pub config: RepositoryConfig,
}
impl EntityRepository {
    pub fn new(config: RepositoryConfig) -> Self {
        EntityRepository {
            executables: Vec::new(),
            documents: Vec::new(),
            generic_executables: Vec::new(),
            config,
        }
    }
    pub fn populate(&mut self) -> &mut Self {
        self.executables.clear();
        self.documents.clear();
        self.generic_executables.clear();

        self.executables = scan_path_executables();
        self.generic_executables = EntityRepository::build_generic_executables(&self.executables);

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
    pub fn build_generic_executables(entities: &Vec<ExecutableEntity>) -> Vec<FileEntity> {
        entities.iter()
            .cloned()
            .map(|e| FileEntity::Executable(e))
            .collect::<Vec<FileEntity>>()
    }

    //
    // documents
    //

    // pub fn get_generic_documents(&self) -> &Vec<FileEntity> {
    //     &self.generic_documents
    // }

    pub fn build_generic_documents(entities: &Vec<RipgrepEntity>) -> Vec<FileEntity> {
        entities.iter()
            .cloned()
            .map(|e| FileEntity::Ripgrep(e))
            .collect::<Vec<FileEntity>>()
    }
}

pub struct RepositoryConfig {
    pub exec_paths: Vec<PathBuf>,
    pub rg_paths: Vec<PathBuf>,
}
