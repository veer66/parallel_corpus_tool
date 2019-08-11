#[derive(Debug, Clone)]
pub struct PhraseRangeWithTokIndex {
    pub s: usize,
    pub e: usize,
}

#[derive(Debug)]
pub struct PhrasePairWithTokIndex {
    pub source: PhraseRangeWithTokIndex,
    pub target: PhraseRangeWithTokIndex,
}
