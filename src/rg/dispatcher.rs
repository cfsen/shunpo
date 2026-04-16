use crate::search::entity_model::{self, CustomDispatcher};

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
