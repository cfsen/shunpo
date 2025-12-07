use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::{ hyprland::error::HyprError, id_type, string_type };

//
// newtype definitions
//
id_type!(MonitorId);
id_type!(WorkspaceId);

string_type!(Floating);
string_type!(KeyboardName);
string_type!(LayoutName);
string_type!(MonitorDesc);
string_type!(MonitorName);
string_type!(Namespace);
string_type!(Owner);
string_type!(PinState);
string_type!(State);
string_type!(SubmapName);
string_type!(WindowAddr);
string_type!(WindowClass);
string_type!(WindowTitle);
string_type!(WorkspaceName);

pub enum FullscreenEvent {
    Exited,
    Entered,
}
impl FullscreenEvent {
    pub fn parse_raw(value: &str) -> Result<FullscreenEvent, HyprError> {
        match value {
            "0" => Ok(FullscreenEvent::Exited),
            "1" =>  Ok(FullscreenEvent::Entered),
            _ => Err(HyprError::EventParseFailed),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: WorkspaceName,
    pub monitor: MonitorName,
    #[serde(rename = "monitorID")]
    pub monitor_id: MonitorId,
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
    pub address: WindowAddr,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub workspace: WorkspaceInfo,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: Option<u32>,
    pub class: WindowClass,
    pub title: WindowTitle,
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
    pub id: MonitorId,
    pub name: MonitorName,
    pub description: MonitorDesc,
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
    pub solitary_blocked_by: Option<Vec<String>>,
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
pub struct Layers {
    #[serde(flatten)]
    pub monitors: HashMap<String, MonitorLayers>, // monitor name
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MonitorLayers {
    pub levels: Levels,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Levels {
    #[serde(rename = "0")]
    pub background: Vec<Layer>,
    #[serde(rename = "1")]
    pub bottom: Vec<Layer>,
    #[serde(rename = "2")]
    pub top: Vec<Layer>,
    #[serde(rename = "3")]
    pub overlay: Vec<Layer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layer {
    pub address: String,
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub namespace: String,
    pub pid: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceInfo {
    pub id: WorkspaceId,
    pub name: String,
}
