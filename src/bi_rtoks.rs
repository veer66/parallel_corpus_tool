use crate::lang::LangKey;
use crate::rtok::RTok;

const MAX_PHRASE_LEN: usize = 10;

#[derive(Debug, Clone)]
pub struct BiRToks {
    pub source: Vec<RTok>,
    pub target: Vec<RTok>,
}

impl BiRToks {
    pub fn rtoks(&self, lang_key: LangKey) -> Vec<RTok> {
        match lang_key {
            LangKey::SOURCE => self.source.clone(),
            LangKey::TARGET => self.target.clone(),
        }
    }
}
