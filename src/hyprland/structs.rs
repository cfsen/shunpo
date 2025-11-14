use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub monitor: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: i32,
    pub windows: i32,
    #[serde(rename = "hasfullscreen")]
    pub has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    pub last_window: String,
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
    #[serde(rename = "ispersistent")]
    pub is_persistent: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub workspace: WorkspaceInfo,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: Option<u32>,
    pub class: String,
    pub title: String,
    #[serde(rename = "initialClass")]
    pub initial_class: String,
    #[serde(rename = "initialTitle")]
    pub initial_title: String,
    pub pid: u32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: u32,
    #[serde(rename = "fullscreenClient")]
    pub fullscreen_client: u32,
    pub grouped: Vec<String>,
    pub tags: Vec<String>,
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: u32,
    #[serde(rename = "inhibitingIdle")]
    pub inhibiting_idle: bool,
    #[serde(rename = "xdgTag")]
    pub xdg_tag: String,
    #[serde(rename = "xdgDescription")]
    pub xdg_description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "physicalWidth")]
    pub physical_width: i32,
    #[serde(rename = "physicalHeight")]
    pub physical_height: i32,
    #[serde(rename = "refreshRate")]
    pub refresh_rate: f32,
    pub x: i32,
    pub y: i32,
    #[serde(rename = "activeWorkspace")]
    pub active_workspace: WorkspaceInfo,
    #[serde(rename = "specialWorkspace")]
    pub special_workspace: WorkspaceInfo,
    pub reserved: [i32; 4],
    pub scale: f32,
    pub transform: i32,
    pub focused: bool,
    #[serde(rename = "dpmsStatus")]
    pub dpms_status: bool,
    pub vrr: bool,
    pub solitary: String,
    #[serde(rename = "solitaryBlockedBy")]
    pub solitary_blocked_by: Vec<String>,
    #[serde(rename = "activelyTearing")]
    pub actively_tearing: bool,
    #[serde(rename = "tearingBlockedBy")]
    pub tearing_blocked_by: Vec<String>,
    #[serde(rename = "directScanoutTo")]
    pub direct_scanout_to: String,
    #[serde(rename = "directScanoutBlockedBy")]
    pub direct_scanout_blocked_by: Vec<String>,
    pub disabled: bool,
    #[serde(rename = "currentFormat")]
    pub current_format: String,
    #[serde(rename = "mirrorOf")]
    pub mirror_of: String,
    #[serde(rename = "availableModes")]
    pub available_modes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
}
