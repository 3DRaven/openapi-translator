use anyhow::{Context, Result};
use log::error;
use mlua::LuaSerdeExt;
use openapiv3::OpenAPI;
use std::ffi::OsStr;

use crate::{
    check_scripts,
    enums::common::Script,
    holders::context::{
        get_lua_vm, recreate_lua_vm, CLI, DEFAULT_TESTS_EXPECTED_DIR_NAME,
        DEFAULT_TESTS_OPENAPI_DIR_NAME, DEFAULT_TESTS_OPENAPI_FILE_NAME,
        DEFAULT_TESTS_OUT_DIR_NAME, EXTENSION_TARGET_PARAMETERS_NAME, LOG_CONTEXT,
        NULL_VALUE_VARIABLE_NAME_IN_LUA, TARGET_PARAMETERS_VARIABLE_NAME_IN_LUA,
        TARGET_PATH_VARIABLE_NAME_IN_LUA, VISITORS_PATH_VARIABLE_NAME_IN_LUA,
    },
    structs::common::CallStack,
    Commands,
};

use super::visitors;

pub fn set_global_lua_parameters(openapi: &OpenAPI) -> Result<CallStack> {
    recreate_lua_vm();
    let lua_vm = get_lua_vm();

    openapi
        .extensions
        .get(EXTENSION_TARGET_PARAMETERS_NAME)
        .map(|it| {
            let params_value = lua_vm.to_value(it)?;
            lua_vm
                .globals()
                .set(TARGET_PARAMETERS_VARIABLE_NAME_IN_LUA, params_value)
        })
        .transpose()?;

    CLI.target_parameters
        .as_ref()
        .map(|it| {
            let params_value = lua_vm.to_value(it)?;
            lua_vm
                .globals()
                .set(TARGET_PARAMETERS_VARIABLE_NAME_IN_LUA, params_value)
        })
        .transpose()?;

    let fake_null = lua_vm.null();
    lua_vm
        .globals()
        .set(NULL_VALUE_VARIABLE_NAME_IN_LUA, fake_null)?;

    let visitors_path_str = CLI
        .get_visitors_dir()
        .to_str()
        .expect("unable to get string from visitors path");
    lua_vm.globals().set(
        VISITORS_PATH_VARIABLE_NAME_IN_LUA,
        lua_vm.to_value(visitors_path_str)?,
    )?;

    let target_path_str = CLI
        .get_target_dir()
        .to_str()
        .expect("unable to get string from visitors path");
    lua_vm.globals().set(
        TARGET_PATH_VARIABLE_NAME_IN_LUA,
        lua_vm.to_value(target_path_str)?,
    )?;

    //Add relative paths to scripts to use with lua require
    let code = format!(
        r#"
        package.path = "./{}/?.lua;" .. "./{}/?.lua;" .. package.path
        "#,
        visitors_path_str, target_path_str
    );
    lua_vm.load(&code).exec()?;

    //It is drop of mutex lock
    drop(lua_vm);
    check_scripts()?;
    Script::Target.call_func(None, Some(&Script::Prelude.call_func(None, None)?))
}

pub fn visit_commands() -> Result<()> {
    get_commands()?
        .iter()
        .inspect(|command| {
            if let Commands::Translate {
                test_name: Some(it),
                ..
            } = command
            {
                let mut log_test_name = LOG_CONTEXT.lock().unwrap();
                (*it).clone_into(&mut log_test_name);
            }
        })
        .try_for_each(visitors::visit_command)?;
    Ok(())
}

fn get_commands() -> Result<Vec<Commands>> {
    let commands = match &CLI.command {
        Commands::Test { names, tests } => {
            let commands: Vec<Commands> = tests
                .read_dir()
                .with_context(|| format!("Could not read tests dir [{:?}]", &tests))?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_dir()
                        && (names.as_ref().map_or(true, |names| {
                            path.file_name()
                                .and_then(OsStr::to_str)
                                .map_or(false, |dir_name| names.contains(&dir_name.to_string()))
                        }))
                })
                .map(|test| Commands::Translate {
                    spec: test
                        .join(DEFAULT_TESTS_OPENAPI_DIR_NAME)
                        .join(DEFAULT_TESTS_OPENAPI_FILE_NAME),
                    out: test.join(DEFAULT_TESTS_OUT_DIR_NAME),
                    clean: true,
                    expected: Some(test.join(DEFAULT_TESTS_EXPECTED_DIR_NAME)),
                    test_name: test
                        .as_path()
                        .file_name()
                        .and_then(OsStr::to_str)
                        .map(String::from),
                })
                .collect();

            if commands.is_empty() {
                error!("Could not found any tests");
            }
            commands
        }
        Commands::Translate {
            spec,
            out,
            clean,
            expected,
            test_name: _,
        } => vec![Commands::Translate {
            spec: spec.to_owned(),
            out: out.to_owned(),
            clean: *clean,
            expected: expected.clone(),
            test_name: None,
        }],
    };

    Ok(commands)
}
