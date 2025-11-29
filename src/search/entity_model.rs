use std::path::PathBuf;
use nucleo::Utf32String;

pub enum FileEntity {
    Executable(ExecutableEntity),
    Ripgrep(RipgrepEntity),
    // Image,
    // Audio,
}
#[derive(Clone, Debug)]
pub enum Dispatcher {
    Hyprctl,
    Shell,
    Custom(CustomDispatcher),
}
#[derive(Clone, Debug)]
pub struct CustomDispatcher {
    args: Option<Vec<String>>,
    executable: String,
    from_shell: bool,
}

//
// LauncherEntity
//

#[derive(Clone)]
pub struct LauncherEntity {
    pub alias: String,
    pub command: String,
    pub dispatcher: Dispatcher,
}
impl LauncherEntity {
    pub fn from_executable(entity: &ExecutableEntity) -> Self {
        LauncherEntity {
            alias: entity.ui_name.clone(),
            command: entity.exec.clone(),
            dispatcher: entity.dispatcher.clone(),
        }
    }
    pub fn from_ripgrep(entity: &RipgrepEntity) -> Self {
        LauncherEntity {
            alias: entity.ui_name.clone(),
            command: entity.path.clone().to_string_lossy().to_string(),
            dispatcher: entity.dispatcher.clone(),
        }
    }
}

//
// Executables
//

#[derive(Clone)]
pub struct ExecutableEntity {
    pub dispatcher: Dispatcher,
    pub match_name: Utf32String,
    pub match_rank: Option<u16>,
    pub path: PathBuf,
    pub ui_name: String,

    pub source: ExecutableSource,
    pub exec: String,
}
#[derive(Clone)]
pub enum ExecutableSource {
    DesktopFile,
    PathBinary,
    ShellScript,
}

//
// Ripgrep
//

#[derive(Clone)]
pub struct RipgrepEntity {
    pub dispatcher: Dispatcher,
    pub match_name: Utf32String,
    pub match_rank: Option<u16>,
    pub path: PathBuf,
    pub ui_name: String,
}

//
// FileEntity trait impls
//

impl Entity for FileEntity { }
impl EntityFields for FileEntity {
    fn dispatcher(&self) -> &Dispatcher { 
        match self {
            Self::Executable(e) => &e.dispatcher,
            Self::Ripgrep(r) => &r.dispatcher,
        }
    }
    fn path(&self) -> &PathBuf {
        match self {
            Self::Executable(e) => &e.path,
            Self::Ripgrep(r) => &r.path,
        }
    }
}
impl Matching for FileEntity {
    fn match_field(&self) -> &Utf32String {
        match self {
            Self::Executable(e) => &e.match_name,
            Self::Ripgrep(r) => &r.match_name,
        }
    }
    fn match_rank(&self) -> Option<u16> {
        match self {
            Self::Executable(e) => e.match_rank,
            Self::Ripgrep(r) => r.match_rank,
        }
    }
    fn set_match_rank(&mut self, rank: u16) {
        match self {
            Self::Executable(e) => e.match_rank = Some(rank),
            Self::Ripgrep(r) => r.match_rank = Some(rank),
        }
    }
}
impl Export for FileEntity {
    fn ui_name(&self) -> &String {
        match self {
            Self::Executable(e) => &e.ui_name,
            Self::Ripgrep(r) => &r.ui_name,
        }
    }
    fn into_entity(self) -> FileEntity {
        match self {
            Self::Executable(e) => FileEntity::Executable(e),
            Self::Ripgrep(r) => FileEntity::Ripgrep(r),
        }
    }
    fn into_launcher_entity(&self) -> LauncherEntity {
        match self {
            Self::Executable(e) => { LauncherEntity::from_executable(e) },
            Self::Ripgrep(r) => { LauncherEntity::from_ripgrep(r) },
        }
    }
}

//
// Traits
//

pub trait Entity: EntityFields + Matching + Export { }
pub trait EntityFields {
    fn dispatcher(&self) -> &Dispatcher;
    fn path(&self) -> &PathBuf;
}
pub trait Matching {
    fn match_field(&self) -> &Utf32String;
    fn match_rank(&self) -> Option<u16>;
    fn set_match_rank(&mut self, rank: u16);
}
pub trait Export {
    fn ui_name(&self) -> &String;
    fn into_entity(self) -> FileEntity;
    fn into_launcher_entity(&self) -> LauncherEntity;
}
