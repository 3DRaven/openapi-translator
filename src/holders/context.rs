use std::{
    any::Any,
    collections::HashMap,
    sync::{Mutex, MutexGuard},
};

use clap::Parser;
use log::debug;
use mlua::Lua;
use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use crate::Cli;
use anyhow::{anyhow, Result};

pub const DEFAULT_TESTS_OPENAPI_DIR_NAME: &str = "openapi";
pub const DEFAULT_TESTS_OUT_DIR_NAME: &str = "actual";
pub const DEFAULT_TESTS_EXPECTED_DIR_NAME: &str = "expected";
pub const DEFAULT_TESTS_OPENAPI_FILE_NAME: &str = "openapi.yml";
pub const DEFAULT_LOGS_COLOR_MODE: &str = "always";
pub const DEFAULT_LOGS_LOG_LEVEL: &str = "debug";

pub const EXTENSION_FOR_NAME: &str = "x-ot-name";
pub const EXTENSION_ANY_ADDITIONAL_PROPERTIES_NAME: &str = "x-ot-additional-properties-name";
pub const EXTENSION_TARGET_PARAMETERS_NAME: &str = "x-ot-target-parameters";
pub const TARGET_PARAMETERS_NAME_IN_LUA: &str = "targetParameters";
pub const DEFAULT_OBJECT_ADDITIONAL_PROPERTIES: &str = "additionalProperties";

pub static CLIENT: Lazy<Client> = Lazy::new(reqwest::blocking::Client::new);
pub static CLI: Lazy<&'static Cli> = Lazy::new(|| Box::leak(Box::new(Cli::parse())));
pub static LUA_VM: Lazy<Mutex<Lua>> = Lazy::new(|| Mutex::new(Lua::new()));
pub static LOG_CONTEXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn get_lua_vm() -> MutexGuard<'static, Lua> {
    LUA_VM.lock().expect("Could not lock lua vm")
}

pub fn recreate_lua_vm() {
    let mut lua_vm = LUA_VM.lock().expect("Could not lock lua vm");
    *lua_vm = Lua::new();
}

type Cache = HashMap<String, &'static (dyn Any + Send + Sync)>;
pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| Mutex::new(Cache::new()));

pub fn compute_if_absent<F, R>(key: String, factory: F) -> Result<&'static R>
where
    R: 'static + Any + Send + Sync,
    F: FnOnce() -> Result<R>,
{
    let read_cache = CACHE.lock().expect("Could not lock cache");

    let cached = match read_cache.get(&key) {
        Some(value) => {
            debug!("Used cached value [{}]", key);
            *value
        }
        None => {
            drop(read_cache);
            let leaked_value: &'static R = Box::leak(Box::new(factory()?));
            let mut write_cache = CACHE.lock().expect("Could not lock cache");
            write_cache.insert(key, leaked_value);
            leaked_value
        }
    };

    cached
        .downcast_ref::<R>()
        .ok_or_else(|| anyhow!("Could not cast value to out cached type"))
}
