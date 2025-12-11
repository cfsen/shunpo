use std::{cmp::Reverse, collections::HashMap};

use crate::hyprland::{
    error::HyprError,
    hyprctl::{get_layers, get_monitors, get_workspaces},
    structs::{ LayerLevel, Monitor, MonitorLayers, MonitorName, Namespace, Workspace, WorkspaceId }
};

pub struct HyprlandState {
    pub layers: HashMap<MonitorName, MonitorLayers>,
    pub monitors: HashMap<MonitorName, Monitor>,
    pub workspaces: HashMap<WorkspaceId, Workspace>,
    pub shunpo_namespace: Namespace,
    pub focused_monitor: Option<MonitorName>,
}
//
// constructor
//
impl Default for HyprlandState {
    fn default() -> Self {
        HyprlandState {
            layers: HashMap::<MonitorName, MonitorLayers>::new(),
            monitors: HashMap::<MonitorName, Monitor>::new(),
            workspaces: HashMap::<WorkspaceId, Workspace>::new(),
            shunpo_namespace: Namespace::from("shunpo"),
            focused_monitor: None,
        }
    }
}
//
// rebuild
//
impl HyprlandState {
    /// Replaces current state by calling .populate() to rebuild from hyprctl.
    pub fn rebuild(&mut self) -> Result<(), HyprError> {
        *self = Self::populate()?;
        Ok(())
    }
    /// Query hyprctl to build a snapshot of current Hyprland state.
    pub fn populate() -> Result<HyprlandState, HyprError> {
        // collect monitors, layers and workspaces from hyprctl
        let layers = Self::fetch_hyprctl_layers()?;
        let monitors = Self::fetch_hyprctl_monitors()?;
        let workspaces = Self::fetch_hyprctl_workspaces()?;

        // find focused monitor
        let focused_monitor = monitors
            .iter()
            .find_map(|(mname, monitor)|
                monitor.focused.then_some(mname.clone())
            );

        Ok(HyprlandState {
            layers,
            monitors,
            workspaces,
            shunpo_namespace: Namespace::from("shunpo"),
            focused_monitor,
        })
    }
    pub fn rebuild_monitors(&mut self) -> Result<(), HyprError> {
        let monitors = Self::fetch_hyprctl_monitors()?;
        self.monitors = monitors;
        Ok(())
    }
    pub fn rebuild_workspaces(&mut self) -> Result<(), HyprError> {
        let workspaces = Self::fetch_hyprctl_workspaces()?;
        self.workspaces = workspaces;
        Ok(())
    }
}
//
// hyprctl callers
//
impl HyprlandState {
    /// Fetch workspaces by calling `hyprctl workspaces`
    pub fn fetch_hyprctl_workspaces() -> Result<HashMap<WorkspaceId, Workspace>, HyprError> {
        let hyprtctl_workspaces = get_workspaces().map_err(|_| HyprError::HyprCtlFetchWorkspaces)?;

        Ok(hyprtctl_workspaces
            .into_iter()
            .map(|w| (w.id.clone(), w))
            .collect())
    }
    /// Fetch monitors by calling `hyprctl monitors`
    pub fn fetch_hyprctl_monitors() -> Result<HashMap<MonitorName, Monitor>, HyprError> {
        let hyprctl_monitors = get_monitors().map_err(|_| HyprError::HyprCtlFetchMonitors)?;

        Ok(hyprctl_monitors
            .into_iter()
            .map(|m| (m.name.clone(), m))
            .collect())
    }
    /// Fetch monitor layers
    pub fn fetch_hyprctl_layers() -> Result<HashMap<MonitorName, MonitorLayers>, HyprError> {
        let hyprctl_layers = get_layers().map_err(|_| HyprError::HyprCtlFetchLayers)?;

        Ok(hyprctl_layers.monitors)
    }
}
//
// update
//
impl HyprlandState {
    pub fn update_focused_monitor(&mut self, monitor_name: MonitorName) {
        self.focused_monitor = Some(monitor_name);
    }
}
//
// remove
//
impl HyprlandState {
    pub fn remove_monitor(&mut self, monitor_name: MonitorName) -> Result<Monitor, HyprError> {
        self.monitors.remove(&monitor_name).ok_or(HyprError::MonitorIdNotFound)
    }

    pub fn remove_workspace(&mut self, id: WorkspaceId) -> Result<Workspace, HyprError> {
        self.workspaces.remove(&id).ok_or(HyprError::WorkspaceIdNotFound)
    }
}
//
// shunpo integration
//
impl HyprlandState {
    pub fn get_shunpo_monitor_layer(&self) -> Result<(MonitorName, LayerLevel), HyprError> {
        Self::find_monitor_layer_for_namespace("shunpo", &self.layers)
    }
    pub fn find_monitor_layer_for_namespace(namespace: &str, layers: &HashMap<MonitorName, MonitorLayers>) -> Result<(MonitorName, LayerLevel), HyprError> {
        layers
            .iter()
            .find_map(|(monitor_name, monitor_layers)| {
                [(LayerLevel::Overlay, &monitor_layers.levels.overlay), (LayerLevel::Bottom, &monitor_layers.levels.bottom)]
                    .iter()
                    .find_map(|(layer_level, vec_layer)| {
                        vec_layer
                            .iter()
                            .any(|l| l.namespace == namespace)
                            .then(|| (monitor_name.clone(), layer_level.clone()))
                    })
            })
            .ok_or(HyprError::ShunpoNotFound)
    }
    pub fn shunpo_should_retarget(&self) -> Result<bool, HyprError> {
        let (shunpo_monitor, shunpo_layer) = self.get_shunpo_monitor_layer()?;
        let (target_monitor, target_layer) = self.shunpo_get_target()?;

        Ok(!(&shunpo_monitor == target_monitor && shunpo_layer == target_layer))
    }
    pub fn shunpo_get_target(&self) -> Result<(&MonitorName, LayerLevel), HyprError> {
        if self.monitors.len() < 1 {
            return Err(HyprError::ShunpoTargetNoSolution)
        }

        // (MonitorName, Score) for ranking results
        let mut candidates: Vec<(MonitorName, i8)> = Vec::with_capacity(self.monitors.len());
        for (mname, monitor) in self.monitors.iter() {
            if self.workspaces
                .get(&monitor.active_workspace.id)
                .is_some_and(|w| !w.has_fullscreen)
            {
                candidates.push((mname.clone(), Self::monitor_score_weight(mname)));
            }
        }

        // every monitor is running fullscreen client, send shunpo to bottom of arbitrary monitor
        if candidates.len() < 1 {
            let (mname, _) = self.monitors.iter().next()
                .ok_or(HyprError::ShunpoInvariantAllFullscreen)?;

            return Ok((mname, LayerLevel::Bottom))
        }

        if candidates.len() > 1 {
            candidates.sort_by_key(|(_, score)| Reverse(*score));
        }

        // send shunpo to overlay on top scoring monitor
        let mname = self.monitors.get(&candidates[0].0)
            .map(|m| &m.name)
            .ok_or(HyprError::ShunpoInvariantTargetTopScore)?;

        Ok((&mname, LayerLevel::Overlay))
    }

    // TODO: temporary impl pending config file
    fn monitor_score_weight(mname: &MonitorName) -> i8 {
        let mut monitors = HashMap::<MonitorName, i8>::new();
        monitors.insert(MonitorName::from("DP-3"), 3);
        monitors.insert(MonitorName::from("DP-2"), 2);
        monitors.insert(MonitorName::from("DP-1"), 1);

        monitors.get(&mname).copied().unwrap_or(0)
    }
}
