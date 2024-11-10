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
pub struct Code {
    pub code: Option<String>,
    pub file: String,
    pub mode: WriteMode,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Call(pub Option<String>, pub Script);

pub fn get_call_id(schema_name: Option<&str>, reference: &String) -> Option<String> {
    Some(schema_name.map_or_else(
        || reference.to_owned(),
        |name| format!("{}->{}", name, reference),
    ))
}

#[derive(Clone)]
pub struct CallStack {
    calls: Vec<Call>,
}

impl CallStack {
    pub fn and_then<F: FnOnce(&CallStack) -> Result<CallStack>>(self, op: F) -> Result<CallStack> {
        op(&self)
    }

    pub fn new(call: Call) -> Self {
        CallStack { calls: vec![call] }
    }
}

impl Add<Call> for CallStack {
    type Output = Self;

    fn add(self, call: Call) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(call);
        CallStack { calls: combined }
    }
}

impl Add<&Call> for CallStack {
    type Output = Self;

    fn add(self, call: &Call) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(call.clone());
        CallStack { calls: combined }
    }
}

impl Add<&Call> for &CallStack {
    type Output = CallStack;

    fn add(self, call: &Call) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(call.clone());
        CallStack { calls: combined }
    }
}

impl Add<Call> for &CallStack {
    type Output = CallStack;

    fn add(self, call: Call) -> Self::Output {
        let mut combined = self.calls.clone();
        combined.push(call);
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
