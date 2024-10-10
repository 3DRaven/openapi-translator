use anyhow::{Context, Result};
use log::error;
use mlua::LuaSerdeExt;
use openapiv3::OpenAPI;
use std::ffi::OsStr;

use crate::{
    holders::context::{
        get_lua_vm, recreate_lua_vm, CLI, DEFAULT_EXTENSION_TARGET_PARAMETERS_NAME,
        DEFAULT_EXTENSION_TARGET_PARAMETERS_NAME_IN_LUA, DEFAULT_TESTS_EXPECTED_DIR_NAME,
        DEFAULT_TESTS_OPENAPI_DIR_NAME, DEFAULT_TESTS_OPENAPI_FILE_NAME,
        DEFAULT_TESTS_OUT_DIR_NAME, LOG_CONTEXT, SCRIPT_PRELUDE,
    },
    Commands,
};

use super::{scripts, visitors};

pub fn set_global_lua_parameters(openapi: &OpenAPI) -> Result<()> {
    recreate_lua_vm();
    let lua_vm = get_lua_vm();

    openapi
        .extensions
        .get(DEFAULT_EXTENSION_TARGET_PARAMETERS_NAME)
        .map(|it| {
            let params_value = lua_vm.to_value(it)?;
            lua_vm.globals().set(
                DEFAULT_EXTENSION_TARGET_PARAMETERS_NAME_IN_LUA,
                params_value,
            )
        })
        .transpose()?;

    CLI.target_parameters
        .as_ref()
        .map(|it| {
            let params_value = lua_vm.to_value(it)?;
            lua_vm.globals().set(
                DEFAULT_EXTENSION_TARGET_PARAMETERS_NAME_IN_LUA,
                params_value,
            )
        })
        .transpose()?;

    let fake_null = lua_vm.null();
    lua_vm.globals().set("null", fake_null)?;
    drop(lua_vm);

    scripts::call_func(SCRIPT_PRELUDE)
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
        Commands::Test { names } => {
            let test_path = CLI.get_tests_dir();
            let commands: Vec<Commands> = test_path
                .read_dir()
                .with_context(|| format!("Could not read tests dir [{:?}]", &test_path))?
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
