use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub mod bin;
pub mod errors;

pub struct Searcher {
    bins: Vec<bin::Bin>,
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            bins: bin::get_bins(),
        }
    }

    pub fn sorted_bins(&self, search: &str) -> Vec<&bin::Bin> {
        let matcher = SkimMatcherV2::default().smart_case().use_cache(true);

        let mut bins = Vec::new();
        for i in &self.bins {
            if matcher.fuzzy_match(i.name(), search).unwrap_or(0) > 30 {
                bins.push(i);
            }
        }

        //we do a bitwise not because sort() gives smaller values higher positions.
        bins.sort_by_cached_key(|bin| !matcher.fuzzy_match(bin.name(), search).unwrap());

        bins
    }
}
