use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ParentType {
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "ARRAY")]
    Array,
    #[serde(rename = "ADDITIONAL")]
    Additional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriteMode {
    #[serde(rename = "APPEND")]
    Append,
    #[serde(rename = "PREPEND")]
    Prepend,
    #[serde(rename = "REMOVE")]
    Remove,
}
