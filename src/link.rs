use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Link {
    pub source: usize,
    pub target: usize,
}
