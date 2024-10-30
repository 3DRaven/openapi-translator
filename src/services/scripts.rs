use anyhow::{Context, Result};
use mlua::{Function, Lua};
use std::fs;

use crate::{enums::common::Script, holders::context::CLI};

//TODO: cached compilation
pub fn get_lua_function<'a>(script: &Script, lua: &'a Lua) -> Result<Function<'a>> {
    let scripts_dir = match script {
        Script::Target => CLI.get_target_dir(),
        _ => CLI.get_visitors_dir(),
    };
    let script_relative_path: &str = script.into();
    let script_path = scripts_dir.join(format!("{}.lua", script_relative_path));
    let script_code = fs::read_to_string(&script_path)
        .with_context(|| format!("Could not read lua script [{:?}]", &script_path))?;
    let chunk = lua.load(script_code);
    chunk
        .eval()
        .with_context(|| format!("Could not compile lua script [{}]", script))
}
