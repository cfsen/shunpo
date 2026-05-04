use std::{collections::HashMap, fmt::Display, path::PathBuf};
use nucleo::Utf32String;

#[derive(Clone)]
pub enum FileEntity {
    Executable(ExecutableEntity),
    Ripgrep(RipgrepEntity),
    Virtual(VirtualEntity),
    // Image,
    // Audio,
}
#[derive(Clone, Debug)]
pub enum Dispatcher {
    Hyprctl,
    Shell,
    Virtual,
    Custom,
}
impl Display for Dispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dispatcher::Hyprctl => write!(f, "Hyprctl"),
            Dispatcher::Shell => write!(f, "Shell"),
            Dispatcher::Virtual => write!(f, "Virtual"),
            Dispatcher::Custom => write!(f, "Custom"),
        }
    }
}
#[derive(Clone, Debug)]
pub struct CustomDispatcher {
    pub alias: String,          // for identifying this dispatcher when outputting errors
    pub requires: Vec<String>,  // args it expects to parse: $term, $editor, $path, etc.
    pub template: String,       // "hyprctl dispatch exec \"$term -e $editor $path"
    pub valid: bool,            // true when `template` contains every `require`. set with validate_template()
}
impl CustomDispatcher {
    // TODO: result type
    pub fn compose_dispatch(&self, args: HashMap<String, &str>) -> Option<String> {
        if !self.valid { return None; }

        let mut dispatch_call = self.template.clone();
        for req in &self.requires {
            let Some(to) = args.get(req) else { return None; };
            dispatch_call = dispatch_call.replace(req, to);
        }
        Some(dispatch_call)
    }
    pub fn validate_template(&mut self) -> bool {
        for req in &self.requires {
            if self.template.find(req) == None {
                self.valid = false;
                return self.valid;
            }
        }
        self.valid = true;
        self.valid
    }
}

//
// LauncherEntity
//

#[derive(Clone)]
pub struct LauncherEntity {
    pub command: String,
    pub dispatcher: Dispatcher,
    pub file_entity: FileEntity,
}
impl LauncherEntity {
    pub fn from_executable(entity: &ExecutableEntity) -> Self {
        let command = match entity.source {
            ExecutableSource::DesktopFile => { entity.exec.clone() },
            _  => { entity.path.to_string_lossy().to_string() },
        };
        LauncherEntity {
            command,
            dispatcher: entity.dispatcher.clone(),
            file_entity: FileEntity::Executable(entity.to_owned()),
        }
    }
    pub fn from_ripgrep(entity: &RipgrepEntity) -> Self {
        LauncherEntity {
            command: entity.path.clone().to_string_lossy().to_string(),
            dispatcher: entity.dispatcher.clone(),
            file_entity: FileEntity::Ripgrep(entity.to_owned()),
        }
    }
    pub fn from_virtual(entity: &VirtualEntity) -> Self {
        LauncherEntity {
            command: "".into(),
            dispatcher: entity.dispatcher.clone(),
            file_entity: FileEntity::Virtual(entity.to_owned()),
        }
    }
}

//
// Virtual
//
#[derive(Clone)]
pub struct VirtualEntity {
    pub dispatcher: Dispatcher,
    pub match_name: Utf32String,
    #[allow(dead_code)] // TODO: pending impl of weight-by-use results
    pub match_rank: Option<u16>,
    pub path: PathBuf,
    pub ui_name: String,
}
impl VirtualEntity {
    pub fn no_dispatch(text: String) -> Self {
        VirtualEntity {
            dispatcher: Dispatcher::Virtual,
            match_name: "".into(),
            match_rank: None,
            path: PathBuf::new(),
            ui_name: text,
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
    #[allow(dead_code)] // TODO: pending impl of weight-by-use results
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
    #[allow(dead_code)] // TODO: pending impl of weight-by-use results
    pub match_rank: Option<u16>,
    pub path: PathBuf,
    pub ui_name: String,

    pub line: i32,
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
            Self::Virtual(v) => &v.dispatcher,
        }
    }
    fn path(&self) -> &PathBuf {
        match self {
            Self::Executable(e) => &e.path,
            Self::Ripgrep(r) => &r.path,
            Self::Virtual(v) => &v.path,
        }
    }
}
impl Matching for FileEntity {
    fn match_field(&self) -> &Utf32String {
        match self {
            Self::Executable(e) => &e.match_name,
            Self::Ripgrep(r) => &r.match_name,
            Self::Virtual(v) => &v.match_name,
        }
    }
    fn match_rank(&self) -> Option<u16> {
        match self {
            Self::Executable(e) => e.match_rank,
            Self::Ripgrep(r) => r.match_rank,
            Self::Virtual(v) => v.match_rank,
        }
    }
    fn set_match_rank(&mut self, rank: u16) {
        match self {
            Self::Executable(e) => e.match_rank = Some(rank),
            Self::Ripgrep(r) => r.match_rank = Some(rank),
            Self::Virtual(v) => v.match_rank = Some(rank),
        }
    }
}
impl Export for FileEntity {
    fn ui_name(&self) -> &String {
        match self {
            Self::Executable(e) => &e.ui_name,
            Self::Ripgrep(r) => &r.ui_name,
            Self::Virtual(v) => &v.ui_name,
        }
    }
    fn into_entity(self) -> FileEntity {
        match self {
            Self::Executable(e) => FileEntity::Executable(e),
            Self::Ripgrep(r) => FileEntity::Ripgrep(r),
            Self::Virtual(v) => FileEntity::Virtual(v),
        }
    }
    fn into_launcher_entity(&self) -> LauncherEntity {
        match self {
            Self::Executable(e) => { LauncherEntity::from_executable(e) },
            Self::Ripgrep(r) => { LauncherEntity::from_ripgrep(r) },
            Self::Virtual(v) => { LauncherEntity::from_virtual(v) },
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
    #[allow(dead_code)] // TODO: pending impl of weight-by-use results
    fn match_rank(&self) -> Option<u16>;
    #[allow(dead_code)] // TODO: pending impl of weight-by-use results
    fn set_match_rank(&mut self, rank: u16);
}
pub trait Export {
    fn ui_name(&self) -> &String;
    fn into_entity(self) -> FileEntity;
    fn into_launcher_entity(&self) -> LauncherEntity;
}
