use gtk4::{
    ListBox,

    prelude::*,
};

use crate::{
    search::entity_model::LauncherEntity,
    ui_gtk4::{
        types::ShunpoState,
    },
};


pub fn result_data_from_idx(results: &ListBox, state: &ShunpoState) -> Option<LauncherEntity> {
    results.selected_row()
        .and_then(|r| { state.results_data.get(r.index() as usize) })
        .map(|entity| entity.clone())
}
