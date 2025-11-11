use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub address: String,
    pub title: String,
    pub class: String,
    pub workspace: WorkspaceInfo,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
}
