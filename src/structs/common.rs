use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::enums::common::WriteMode;

pub struct ParsedSpec {
    pub path: PathBuf,
    pub spec: Arc<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelName {
    pub base: String,
    pub extended: Option<serde_json::Value>,
}

impl ModelName {
    pub fn new(base: String) -> Self {
        Self {
            base,
            extended: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: Option<String>,
    pub file: String,
    pub mode: WriteMode,
}
