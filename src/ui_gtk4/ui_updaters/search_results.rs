use gtk4::prelude::*;
use gtk4::{
    Label, Box, Orientation,
    ListBoxRow
};

use crate::{coordinator::types::SearchMessageData, ui_gtk4::types::{ShunpoState, ShunpoWidgets}};

pub fn update_results(
    data: SearchMessageData,
    widgets: &ShunpoWidgets,
    state: &mut ShunpoState,
) {
    crate::ui_gtk4::ui_updaters::common::clear_results(widgets);

    // update state
    state.results_data = data.results.clone();

    if data.results.len() == 0 {
        return;
    }

    // populate new results
    for entity  in data.results {
        let row = ListBoxRow::new();
        let box_ = Box::new(Orientation::Horizontal, 10);
        box_.set_margin_top(5);
        box_.set_margin_bottom(5);
        box_.set_margin_start(10);

        let label = Label::new(Some(&entity.alias.to_string()));
        box_.append(&label);

        row.set_child(Some(&box_));
        widgets.results.append(&row);
    }

    // select first result
    if let Some(target_row_idx) = widgets.results.row_at_index(
        widgets.results.selected_row().map_or_else(|| 0, |row| row.index())
    ){
        widgets.results.select_row(Some(&target_row_idx));
    }
}
