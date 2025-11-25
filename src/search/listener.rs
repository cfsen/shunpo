use std::cmp::Reverse;

use log::info;
use tokio::sync::mpsc;
use nucleo::{self, Utf32String};

use crate::{
    coordinator::types::{
        CoordinatorMessage,
        SearchMessageData
    },
    search::{
        item_types::{Executable, SearchItems},
        matcher_helpers::fuzzy_search,
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
    let items = SearchItems::new();
    let mut matcher = nucleo::Matcher::new(nucleo::Config::DEFAULT);

    loop {tokio::select! {
        Some(msg) = search_rx.recv() => {
            let needle = Utf32String::from(msg.clone());
            let needle_view = needle.slice(..);

            // TODO: check search prefix for which type to search
            let mut scored = fuzzy_search::<Executable>(&items.executables, needle_view, &mut matcher);
            scored.sort_by_key(|(score, _)| Reverse(*score));
            scored.truncate(10);

            let success = scored.len() > 0;
            let results: Vec<(u16, Executable)> = scored
                .into_iter()
                .map(|(score,res)| (score, res.clone()))
                .collect();

            info!("search listener received message");
            // TODO: TODO_GENERIC_RESULTS
            let _ = search_coord_tx.send(CoordinatorMessage::SearchMessage(SearchMessageData{
                success,
                results
            }));
        }
        else => {
            info!("Search channel closed, exiting listener.");
            break;
        }
    }}
}
