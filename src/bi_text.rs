use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BiText {
    pub source: String,
    pub target: String,
}
