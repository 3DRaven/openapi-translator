use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::enums::common::{Script, WriteMode};

pub struct BracketScripts {
    pub start: Script,
    pub end: Script,
}

pub struct ParsedSpec {
    pub path: PathBuf,
    pub spec: Arc<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: Option<String>,
    pub file: String,
    pub mode: WriteMode,
}
