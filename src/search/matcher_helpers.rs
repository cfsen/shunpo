use std::cmp::Reverse;

use nucleo::{Matcher, Utf32String};
use crate::search::entity_model::{Entity, LauncherEntity};

pub fn search_entity<'a, T>(
    haystack: &'a Vec<T>,
    needle: String,
    matcher: &mut Matcher
) -> Vec<LauncherEntity> where T: Entity {
    let fuzzy_needle = Utf32String::from(needle);
    let needle_view = fuzzy_needle.slice(..);

    let mut scored: Vec<(u16, &T)> = haystack.iter()
        .filter_map(|entity| {
            matcher.fuzzy_match(
                entity.match_field().slice(..),
                needle_view
            ).map(|score| (match_score_weighting(score), entity))
        }).collect();
    scored.sort_by_key(|(score, _)| Reverse(*score));
    scored.truncate(10);

    let results: Vec<LauncherEntity> = scored.into_iter()
        .map(|(_score, entity)| {
            entity.into_launcher_entity()
        }).collect();

    results
}

fn match_score_weighting(score: u16) -> u16 {
    // TODO: use the match_rank field from entity to modify score 
    score
}
