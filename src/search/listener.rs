use log::info;
use tokio::sync::mpsc;
use nucleo;

use crate::{
    coordinator::types::{
        CoordinatorMessage,
        SearchMessageData
    },
    search::{
        entity_model::FileEntity,
        entity_repository::{EntityRepository, RepositoryConfig},
        matcher_helpers::search_entity
    },
};

pub fn setup_search_listener(
    search_rx: mpsc::UnboundedReceiver<String>,
    search_coord_tx: mpsc::UnboundedSender<CoordinatorMessage>
){
    tokio::spawn(async {
        search_listener(search_rx, search_coord_tx).await;
    });
}
async fn search_listener(
    mut search_rx: mpsc::UnboundedReceiver<String>,
    search_coord_tx: mpsc::UnboundedSender<CoordinatorMessage>
){
    let mut matcher = nucleo::Matcher::new(nucleo::Config::DEFAULT);

    // TODO: populate custom paths
    let repo_config = RepositoryConfig {
        exec_paths: Vec::new(),
        rg_paths: Vec::new(),
    };
    let mut entity_repo = EntityRepository::new(repo_config);
    entity_repo.populate();
    let mut haystack: &Vec<FileEntity>;

    loop {tokio::select! {
        Some(msg) = search_rx.recv() => {
            // skip empty search queries
            if msg.is_empty() {
                let _ = search_coord_tx.send(CoordinatorMessage::SearchMessage(SearchMessageData {
                    success: false,
                    results: Vec::new(),
                }));
                continue;
            }
            // else if msg.starts_with("rg ") {
            //     haystack = entity_repo.get_generic_documents();
            // }
            else {
                haystack = entity_repo.get_generic_executables();
            }

            let results = search_entity(&haystack, msg, &mut matcher);
            let _ = search_coord_tx.send(CoordinatorMessage::SearchMessage(SearchMessageData {
                success: true,
                results,
            }));
        }
        else => {
            info!("Search channel closed, exiting listener.");
            break;
        }
    }}
}
