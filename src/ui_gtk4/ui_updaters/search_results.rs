use gtk4::prelude::*;
use gtk4::{
    Label, Box, Orientation,
    ListBoxRow
};

use crate::search::entity_model::{ExecutableEntity, FileEntity, RipgrepEntity, VirtualEntity};
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
        let row = match entity.file_entity {
            FileEntity::Executable(executable_entity) => {
                row_from_exec(&executable_entity)
            },
            FileEntity::Ripgrep(ripgrep_entity) => {
                row_from_rg(&ripgrep_entity)
            },
            FileEntity::Virtual(virtual_entity) => {
                row_from_virt(&virtual_entity)
            },
        };

        widgets.results.append(&row);
    }

    // select first result
    if let Some(target_row_idx) = widgets.results.row_at_index(
        widgets.results.selected_row().map_or_else(|| 0, |row| row.index())
    ){
        widgets.results.select_row(Some(&target_row_idx));
    }
}

fn row_from_exec(entity: &ExecutableEntity) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = new_listbox_hbox();

    let label = Label::new(Some(&entity.ui_name));
    hbox.append(&label);

    row.set_child(Some(&hbox));
    row
}

fn row_from_rg(entity: &RipgrepEntity) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = new_listbox_hbox();

    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.set_hexpand(true);

    let label = Label::new(Some(&entity.ui_name));
    label.set_halign(gtk4::Align::Start);
    vbox.append(&label);

    let matching_line = format!("{}: {}", entity.line, entity.match_name);
    let body_text = Label::new(Some(&matching_line));
    body_text.set_halign(gtk4::Align::Fill);
    body_text.set_wrap(true);
    body_text.set_xalign(0.0);

    vbox.append(&body_text);
    hbox.append(&vbox);

    row.set_child(Some(&hbox));
    row
}

fn row_from_virt(entity: &VirtualEntity) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = new_listbox_hbox();

    let label = Label::new(Some(&entity.ui_name));
    hbox.append(&label);

    row.set_child(Some(&hbox));
    row
}

fn new_listbox_hbox() -> Box {
    let hbox = Box::new(Orientation::Horizontal, 10);
    hbox.set_margin_top(5);
    hbox.set_margin_bottom(5);
    hbox.set_margin_start(10);
    hbox.set_hexpand(true);
    hbox
}
