use std::{ops::Add, path::PathBuf, sync::Arc};

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use crate::enums::common::{Script, WriteMode};
use anyhow::Result;

pub struct BracketScripts {
    pub start: Script,
    pub end: Script,
}

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

pub struct CallStack {
    calls: Vec<Script>,
}

impl CallStack {
    pub fn and_then<F: FnOnce(&CallStack) -> Result<()>>(self, op: F) -> Result<CallStack> {
        op(&self)?;
        Ok(self)
    }

    pub fn new(root: Script) -> Self {
        CallStack { calls: vec![root] }
    }
}

impl Add<Script> for CallStack {
    type Output = Self;

    fn add(self, script: Script) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(script);
        CallStack { calls: combined }
    }
}

impl Add<&Script> for CallStack {
    type Output = Self;

    fn add(self, script: &Script) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(script.clone());
        CallStack { calls: combined }
    }
}

impl Add<&Script> for &CallStack {
    type Output = CallStack;

    fn add(self, script: &Script) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(script.clone());
        CallStack { calls: combined }
    }
}

impl Serialize for CallStack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.calls.len()))?;
        for call in &self.calls {
            seq.serialize_element(call)?;
        }
        seq.end()
    }
}
