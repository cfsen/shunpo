use crate::{config::config::ShunpoConfig, search::entity_model::{self, CustomDispatcher}};

pub fn create_default() -> CustomDispatcher {
    let mut rg_dispatcher = entity_model::CustomDispatcher {
        alias: "Ripgrep dispatcher".to_string(),
        requires: vec![
            "$term".to_string(),
            "$editor".to_string(),
            "$path".to_string(),
            "$line".to_string(),
        ],
        template: "hyprctl dispatch exec \"$term -e $editor -c $line $path\"".to_string(),
        valid: false,
    };
    rg_dispatcher.validate_template();
    rg_dispatcher
}

pub fn create_from_config(config: &ShunpoConfig) -> Option<CustomDispatcher> {
    let mut rg_dispatcher = entity_model::CustomDispatcher {
        alias: "Ripgrep dispatcher".to_string(),
        requires: vec![
            "$term".to_string(),
            "$editor".to_string(),
            "$path".to_string(),
            "$line".to_string(),
        ],
        template: config.editor_dispatch.clone(),
        valid: false,
    };

    if rg_dispatcher.validate_template() {
        Some(rg_dispatcher)
    }
    else {
        None
    }
}

pub fn from_config_or_default(config: &ShunpoConfig) -> CustomDispatcher {
    if let Some(dispatcher) = create_from_config(&config) {
        dispatcher
    }
    else {
        log::warn!("Using default dispatcher for ripgrep");
        create_default()
    }
}
