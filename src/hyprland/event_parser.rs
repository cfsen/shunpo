use crate::hyprland::{
    error::HyprError,
    structs::{
        Floating,
        FullscreenEvent,
        KeyboardName,
        LayoutName,
        MonitorDesc,
        MonitorId,
        MonitorName,
        Namespace,
        Owner,
        PinState,
        State,
        SubmapName,
        WindowAddr,
        WindowClass,
        WindowTitle,
        WorkspaceId,
        WorkspaceName
    }};

#[allow(dead_code)]
pub enum HyprlandEvent {
    Workspace { wid: WorkspaceId },
    Workspacev2 { wid: WorkspaceId, wname: WorkspaceName },
    Focusedmon { mname: MonitorName, wname: WorkspaceName },
    Focusedmonv2 { mname: MonitorName, wid: WorkspaceId },
    Activewindow { winclass: WindowClass, wintitle: WindowTitle },
    Activewindowv2 { winaddr: WindowAddr },
    Fullscreen { fstate: FullscreenEvent },
    Monitorremoved { mname: MonitorName },
    Monitorremovedv2 { mid: MonitorId, mname: MonitorName, mdesc: MonitorDesc },
    Monitoradded { mname: MonitorName },
    Monitoraddedv2 { mid: MonitorId, mname: MonitorName, mdesc: MonitorDesc},
    Createworkspace { wname: WorkspaceName},
    Createworkspacev2 { wid: WorkspaceId, wname: WorkspaceName },
    Destroyworkspace { wname: WorkspaceName },
    Destroyworkspacev2{ wid: WorkspaceId, wname: WorkspaceName },
    Moveworkspace { wname: WorkspaceName, mname: MonitorName },
    Moveworkspacev2 { wid: WorkspaceId, wname: WorkspaceName, mname: MonitorName },
    Renameworkspace { wid: WorkspaceId, new_name: WorkspaceName },
    Activespecial { wname: WorkspaceName, mname: MonitorName },
    Activespecialv2 { wid: WorkspaceId, wname: WorkspaceName, mname: MonitorName },
    Activelayout { kname: KeyboardName, lname: LayoutName},
    Openwindow { waddr: WindowAddr, wname: WorkspaceName, winclass: WindowClass, wintitle: WindowTitle},
    Closewindow { waddr: WindowAddr },
    Movewindow { waddr: WindowAddr, wname: WorkspaceName},
    Movewindowv2 { waddr: WindowAddr, wid: WorkspaceId, wname: WorkspaceName },
    Openlayer { nspace: Namespace },
    Closelayer { nspace: Namespace },
    Submap { smname: SubmapName },
    Changefloatingmode { waddr: WindowAddr, float: Floating},
    Urgent { waddr: WindowAddr },
    Screencast { state: State, owner: Owner},
    Windowtitle { waddr: WindowAddr },
    Windowtitlev2 { waddr: WindowAddr, wintitle: WindowTitle},
    Togglegroup, // TODO: ref docs
    Moveintogroup { waddr: WindowAddr },
    Moveoutofgroup { waddr: WindowAddr },
    Ignoregrouplock, // TODO: ref docs
    Lockgroups, // TODO: ref docs
    Configreloaded,
    Pin { waddr: WindowAddr, pstate: PinState},
    Minimized { waddr: WindowAddr, action: i8},
    Bell { waddr: WindowAddr },

    IgnoredEvent,
}
impl HyprlandEvent {
    pub fn parse_event(raw: &str) -> Result<Self, HyprError> {
        let (event, value) = raw.split_once(">>").ok_or(HyprError::EventParseFailed)?;
        let event_delimiter = ",";

        match event {
            "workspace" => {
                Ok(HyprlandEvent::Workspace { wid: WorkspaceId::try_from(value)? })
            },
            "workspacev2" => {
                let (wid, wname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Workspacev2 {
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                })
            },
            "focusedmon" => {
                let (mname, wname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Focusedmon {
                    mname: MonitorName::from(mname),
                    wname: WorkspaceName::from(wname),
                })
            },
            "focusedmonv2" => {
                let (mname, wid) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Focusedmonv2 {
                    mname: MonitorName::from(mname),
                    wid: WorkspaceId::try_from(wid)?,
                })
            },
            "activewindow" => {
                let (winclass, wintitle) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Activewindow { 
                    winclass: WindowClass::from(winclass),
                    wintitle: WindowTitle::from(wintitle),
                })
            },
            "activewindowv2" => {
                Ok(HyprlandEvent::Activewindowv2 { winaddr: WindowAddr::from(value) })
            },
            "fullscreen" => {
                Ok(HyprlandEvent::Fullscreen { fstate: FullscreenEvent::parse_raw(value)?})
            },
            "monitorremoved" => {
                Ok(HyprlandEvent::Monitorremoved { mname: MonitorName::from(value) })
            },
            "monitorremovedv2" => {
                let (mid, mname, mdesc) = value_split_twice(value, event_delimiter)?;

                Ok(HyprlandEvent::Monitorremovedv2 {
                    mid: MonitorId::try_from(mid)?,
                    mname: MonitorName::from(mname),
                    mdesc: MonitorDesc::from(mdesc),
                })
            },
            "monitoradded" => {
                Ok(HyprlandEvent::Monitoradded { mname: MonitorName::from(value) })
            },
            "monitoraddedv2" => {
                let (mid, mname, mdesc) = value_split_twice(value, event_delimiter)?;

                Ok(HyprlandEvent::Monitorremovedv2 {
                    mid: MonitorId::try_from(mid)?,
                    mname: MonitorName::from(mname),
                    mdesc: MonitorDesc::from(mdesc),
                })
            },
            "createworkspace" => {
                Ok(HyprlandEvent::Createworkspace { wname: WorkspaceName::from(value) })
            },
            "createworkspacev2" => {
                let (wid, wname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Createworkspacev2 {
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                })
            },
            "destroyworkspace" => {
                Ok(HyprlandEvent::Destroyworkspace { wname: WorkspaceName::from(value) })
            },
            "destroyworkspacev2" => {
                let (wid, wname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Destroyworkspacev2 {
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                })
            },
            "moveworkspace" => {
                let (wname, mname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Moveworkspace {
                    wname: WorkspaceName::from(wname),
                    mname: MonitorName::from(mname),
                })
            },
            "moveworkspacev2" => {
                let (wid, wname, mname) = value_split_twice(value, event_delimiter)?;

                Ok(HyprlandEvent::Moveworkspacev2 {
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                    mname: MonitorName::from(mname),
                })
            },
            "renameworkspace" => {
                let (wid, new_name) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Renameworkspace {
                    wid: WorkspaceId::try_from(wid)?,
                    new_name: WorkspaceName::from(new_name),
                })
            },
            "activespecial" => {
                let (wname, mname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Activespecial {
                    wname: WorkspaceName::from(wname),
                    mname: MonitorName::from(mname),
                })
            },
            "activespecialv2" => {
                let (wid, wname, mname) = value_split_twice(value, event_delimiter)?;

                Ok(HyprlandEvent::Activespecialv2 {
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                    mname: MonitorName::from(mname),
                })
            },
            "activelayout" => {
                let (kname, lname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Activelayout {
                    kname: KeyboardName::from(kname),
                    lname: LayoutName::from(lname),
                })
            },
            "openwindow" => {
                let (waddr, wname, winclass, wintitle) = value_split_thrice(value, event_delimiter)?;

                Ok(HyprlandEvent::Openwindow {
                    waddr: WindowAddr::from(waddr),
                    wname: WorkspaceName::from(wname),
                    winclass: WindowClass::from(winclass),
                    wintitle: WindowTitle::from(wintitle),
                })
            },
            "closewindow" => {
                Ok(HyprlandEvent::Closewindow {
                    waddr: WindowAddr::from(value),
                })
            },
            "movewindow" => {
                let (waddr, wname) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Movewindow {
                    waddr: WindowAddr::from(waddr),
                    wname: WorkspaceName::from(wname),
                })
            },
            "movewindowv2" => {
                let (waddr, wid, wname) = value_split_twice(value, event_delimiter)?;

                Ok(HyprlandEvent::Movewindowv2 {
                    waddr: WindowAddr::from(waddr),
                    wid: WorkspaceId::try_from(wid)?,
                    wname: WorkspaceName::from(wname),
                })
            },
            "openlayer" => {
                Ok(HyprlandEvent::Openlayer {
                    nspace: Namespace::from(value),
                })
            },
            "closelayer" => {
                Ok(HyprlandEvent::Closelayer {
                    nspace: Namespace::from(value),
                })
            },
            "submap" => {
                Ok(HyprlandEvent::Submap {
                    smname: SubmapName::from(value),
                })
            },
            "changefloatingmode" => {
                let (waddr, float) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Changefloatingmode {
                    waddr: WindowAddr::from(waddr),
                    float: Floating::from(float),
                })
            },
            "urgent" => {
                Ok(HyprlandEvent::Urgent {
                    waddr: WindowAddr::from(value),
                })
            },
            "screencast" => {
                let (state, owner) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Screencast {
                    state: State::from(state),
                    owner: Owner::from(owner),
                })
            },
            "windowtitle" => {
                Ok(HyprlandEvent::Windowtitle {
                    waddr: WindowAddr::from(value),
                })
            },
            "windowtitlev2" => {
                let (waddr, wintitle) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Windowtitlev2 {
                    waddr: WindowAddr::from(waddr),
                    wintitle: WindowTitle::from(wintitle),
                })
            },
            "togglegroup" => {
                Ok(HyprlandEvent::Togglegroup)
            },
            "moveintogroup" => {
                Ok(HyprlandEvent::Moveintogroup {
                    waddr: WindowAddr::from(value),
                })
            },
            "moveoutofgroup" => {
                Ok(HyprlandEvent::Moveoutofgroup {
                    waddr: WindowAddr::from(value),
                })
            },
            "ignoregrouplock" => {
                Ok(HyprlandEvent::Ignoregrouplock)
            },
            "lockgroups" => {
                Ok(HyprlandEvent::Lockgroups)
            },
            "configreloaded" => {
                Ok(HyprlandEvent::Configreloaded)
            },
            "pin" => {
                let (waddr, pstate) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Pin {
                    waddr: WindowAddr::from(waddr),
                    pstate: PinState::from(pstate),
                })
            },
            "minimized" => {
                let (waddr, action) = value_split_once(value, event_delimiter)?;

                Ok(HyprlandEvent::Minimized {
                    waddr: WindowAddr::from(waddr),
                    action: action.parse()?,
                })
            },
            "bell" => {
                Ok(HyprlandEvent::Bell {
                    waddr: WindowAddr::from(value),
                })
            },
            _ => {
                Ok(HyprlandEvent::IgnoredEvent)
            }
        }
    }
}
//
// helpers
//
fn value_split_once<'a>(value: &'a str, delimiter: &str) -> Result<(&'a str, &'a str), HyprError> {
    value.split_once(delimiter)
        .ok_or(HyprError::EventParseFailed)
}
fn value_split_twice<'a>(value: &'a str, delimiter: &str) -> Result<(&'a str, &'a str, &'a str), HyprError> {
    value.split_once(delimiter)
        .and_then(|(a, bc)| {
            bc.split_once(delimiter)
                .map(|(b, c)| (a, b, c))
        })
        .ok_or(HyprError::EventParseFailed)
}
fn value_split_thrice<'a>(value: &'a str, delimiter: &str) -> Result<(&'a str, &'a str, &'a str, &'a str), HyprError> {
    value.split_once(delimiter)
        .and_then(|(a, bcd)| {
            bcd.split_once(delimiter)
                .and_then(|(b, cd)| {
                    cd.split_once(delimiter)
                        .map(|(c, d)| (a, b, c ,d))
                })
        })
        .ok_or(HyprError::EventParseFailed)
}
