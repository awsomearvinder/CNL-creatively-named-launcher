use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

mod bin;
pub fn sorted_bins(search: &str) -> Vec<bin::Bin> {
    let mut bins = bin::get_bins();
    let matcher = SkimMatcherV2::default();
    let mut bad_vals = Vec::new();
    bins.sort_by_cached_key(|bin| {
        let score = matcher.fuzzy_match(bin.name(), search);
        if score.is_none() {
            bad_vals.push(bin.clone());
        } else if score.unwrap() < 30 {
            bad_vals.push(bin.clone());
        };
        //this makes all scores negative, in order to make the highest score lowest,
        //and lowest score highest. This is done because sort_by_cached_key sorts the
        //smallest number as highest sort val, and fuzzy match gives lowest score
        //to least likely.
        score.unwrap_or(100_000_000) * -1
    });
    for item in bad_vals.iter() {
        bins.remove_item(item);
    }
    bins
}