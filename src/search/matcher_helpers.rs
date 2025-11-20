use nucleo::{Utf32Str, Matcher};
use crate::search::item_types::MatchField;

pub fn fuzzy_search<'a, T>(
    items: &'a Vec<T>,
    needle: Utf32Str,
    matcher: &mut Matcher
) -> Vec<(u16, &'a T)> where T: MatchField {
    items.iter()
        .filter_map(|item| {
            matcher.fuzzy_match(
                item.get_match_field().slice(..),
                needle
            ).map(|score| (score, item))
        }).collect()
}
