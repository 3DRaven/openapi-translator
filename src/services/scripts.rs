use anyhow::{Context, Result};
use mlua::{Function, Lua};

use crate::enums::common::Script;

pub fn get_call_id(schema_name: Option<&str>, reference: &String) -> Option<String> {
    Some(schema_name.map_or_else(
        || reference.to_owned(),
        |name| format!("{}->{}", name, reference),
    ))
}

pub fn get_lua_function<'a>(script: &Script, lua: &'a Lua) -> Result<Function<'a>> {
    let script_get_command: &str = script.into();
    lua.load(script_get_command)
        .eval()
        .with_context(|| format!("Unable to get LUA function by [{}]", script_get_command))
}
