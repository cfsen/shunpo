use std::collections::HashMap;

use log::{error, info};
use tokio::sync::mpsc;

use crate::{coordinator::{error::CoordinatorError, types::{
    CoordinatorMessage, FeedbackData, GuiMessage, HyprlandEventData, SearchMessageData, ShunpoSocketEventData
}}, hyprland::hyprctl::{dispatch, dispatch_from_term}, search::entity_model::{CustomDispatcher, Dispatcher}};

pub async fn coordinator_run(
    hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    search_coord_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    gui_tx: async_channel::Sender<GuiMessage>,
    feedback_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
) {
    tokio::spawn(async move {
        if let Err(e) = coordinator_listener(
            gui_tx,
            hyprland_rx,
            shunpo_rx,
            search_coord_rx,
            feedback_rx
        ).await {
            error!("Coordinator loop exited with error: {:?}", e);
        }
    });
}

async fn coordinator_listener(
    gui_tx: async_channel::Sender<GuiMessage>,
    mut hyprland_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut shunpo_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut search_coord_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
    mut feedback_rx: mpsc::UnboundedReceiver<CoordinatorMessage>,
) -> Result<(),Box<dyn std::error::Error + Send + Sync>> {

    // TODO: ripgrep dispatcher template from config
    let rg_dispatcher = crate::rg::dispatcher::create_default();

    loop {
        tokio::select! {
            Some(CoordinatorMessage::HyprlandEvent(msg)) = hyprland_rx.recv()
            => { log_error(handle_hyprland(msg, &gui_tx).await, "Hyprland handler"); },

            Some(CoordinatorMessage::ShunpoSocketEvent(msg)) = shunpo_rx.recv()
            => { log_error(handle_shunpo_socket(msg, &gui_tx).await, "Socket handler"); },

            Some(CoordinatorMessage::SearchMessage(msg)) = search_coord_rx.recv()
            => { log_error(handle_search(msg, &gui_tx).await, "Search handler"); },

            Some(CoordinatorMessage::Feedback(msg)) = feedback_rx.recv()
            => { log_error(handle_feedback(msg, &rg_dispatcher, &gui_tx).await, "Feedback handler"); },

            else => {
                info!("All input channels closed. Exiting coordinator loop.");
                break;
            },
        }}
    Ok(())
}

//
// handlers
//

async fn handle_hyprland(
    msg: HyprlandEventData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    gui_tx.send(msg.gui_msg).await?;

    Ok(())
}

async fn handle_shunpo_socket(
    msg: ShunpoSocketEventData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = match msg {
        ShunpoSocketEventData::ToggleUiMode => GuiMessage::ToggleUiMode,
    };

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

async fn handle_search(
    msg: SearchMessageData,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = GuiMessage::DisplayResults(msg);

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

async fn handle_feedback(
    msg: FeedbackData,
    rg_dispatcher: &CustomDispatcher,
    gui_tx: &async_channel::Sender<GuiMessage>,
) -> Result<(), CoordinatorError> {
    let gui_cmd = match msg {
        FeedbackData::GuiMessagePassthrough(g) => { g }
        FeedbackData::Run(run) => {

            // TODO: proper arg parsing
            // ignoring launch args for now
            let cmd = run.command
                .trim_end_matches("%u")
                .trim_end_matches("%U")
                .trim_end_matches("%f")
                .trim().to_string();

            let dispatch = match run.dispatcher {
                Dispatcher::Shell => { dispatch_from_term(&cmd) },
                Dispatcher::Hyprctl => { dispatch(&cmd) },
                Dispatcher::Custom => {
                    match run.file_entity {
                        crate::search::entity_model::FileEntity::Ripgrep(ripgrep_entity) => {
                            // TODO: move to own fn
                            let mut args: HashMap<String, &str> = HashMap::new();
                            let _path = ripgrep_entity.path.to_string_lossy();
                            let _line = ripgrep_entity.line.to_string();

                            args.insert("$term".to_string(), "ghostty");
                            args.insert("$editor".to_string(), "nvim");
                            args.insert("$path".to_string(), &_path);
                            args.insert("$line".to_string(), &_line);

                            if let Some(call) = rg_dispatcher.compose_dispatch(args) {
                                info!("ripgrep dispatcher will call: {:?}", call);
                                let _ = std::process::Command::new("sh")
                                    .arg("-c")
                                    .arg(&call)
                                    .spawn();
                            }
                            else {
                                error!("Failed to compose dispatch for ripgrep.");
                            }
                        },
                        crate::search::entity_model::FileEntity::Executable(_) => {
                            error!("Executable entity requested a custom dispatcher.");
                        },
                    }
                    Ok(())
                },
            };
            match dispatch {
                Ok(_) => {
                    info!("Dispatched: {}", &cmd);
                },
                Err(e) => {
                    error!("Dispatch error: {}", e);
                },
            }

            GuiMessage::Sleep
        },
    };

    gui_tx.send(gui_cmd).await?;
    Ok(())
}

//
// helpers
//

fn log_error<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) {
    if let Err(e) = result {
        error!("{}: {}", context, e);
    }
}
