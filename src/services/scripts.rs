use ::serde::Serialize;
use anyhow::{anyhow, Context, Result};
use log::trace;
use mlua::{FromLuaMulti, Function, Lua, LuaSerdeExt};
use serde_json::json;
use std::{fs, path::Path};

use crate::{
    holders::context::{get_lua_vm, CLI},
    services::{code, scripts},
};

//TODO: cached compilation
pub fn get_lua_function<'a>(script_name: &str, lua: &'a Lua) -> Result<Function<'a>> {
    let script_path = CLI.get_scripts_dir().join(format!("{}.lua", script_name));
    let script = fs::read_to_string(&script_path)
        .with_context(|| format!("Could not read lua script [{:?}]", &script_path))?;
    let chunk = lua.load(&script);
    chunk
        .eval()
        .with_context(|| format!("Could not compile lua script [{}]", &script_name))
}

pub fn call_with_descriptor<T>(out_path: &Path, descriptor: &T, func_name: &str) -> Result<()>
where
    T: Serialize,
{
    let lua_vm = get_lua_vm();
    let func = scripts::get_lua_function(func_name, &lua_vm)?;

    let args_value = serde_json::to_value(descriptor)?;

    let args_vec = if args_value.is_array() {
        args_value
            .as_array()
            .ok_or_else(|| anyhow!("Not a array of args in descriptor"))?
            .iter()
            .map(|it| lua_vm.to_value(it))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![lua_vm.to_value(&args_value)?]
    };

    let code: mlua::Value = match &args_vec {
        args if args.len() == 1 => func.call(&args[0]),
        args if args.len() == 2 => func.call((&args[0], &args[1])),
        args if args.len() == 3 => func.call((&args[0], &args[1], &args[2])),
        args if args.len() == 4 => func.call((&args[0], &args[1], &args[2], &args[3])),
        args if args.len() == 5 => func.call((&args[0], &args[1], &args[2], &args[3], &args[4])),
        _ => {
            panic!(
                "Unknown number of parameters for script [{}] with args [{:?}]",
                &func_name, args_vec
            )
        }
    }
    .with_context(|| {
        format!(
            "Failed to call lua script [{}] with args [{:?}]",
            &func_name, args_vec
        )
    })?;

    code::save_code(out_path, lua_vm.from_value(code)?)?;

    Ok(())
}

pub fn call_func(func_name: &str) -> Result<()> {
    let lua_vm = get_lua_vm();
    let func = scripts::get_lua_function(func_name, &lua_vm)?;
    func.call::<_, ()>(())
        .with_context(|| format!("Could not call lua function [{}]", &func_name))?;
    Ok(())
}

pub fn call_func_and_return<T, R>(func_name: &str, params: &T) -> Result<R>
where
    T: Serialize + ?Sized,
    R: for<'a> FromLuaMulti<'a>,
{
    let lua_vm = get_lua_vm();
    let value = lua_vm.to_value(params)?;
    trace!(
        "Prepared data for Lua function {}:\n{:?}",
        func_name,
        json!(value)
    );
    let func = scripts::get_lua_function(func_name, &lua_vm)?;
    func.call::<_, R>(value).with_context(|| {
        format!(
            "Could not call lua function [{}] with return value",
            &func_name
        )
    })
}
