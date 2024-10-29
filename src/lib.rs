use ansi_term::Color;
use anyhow::{anyhow, Context, Result};
use enums::common::Script;
use env_logger::Env;
use holders::context::{
    get_lua_vm, CLI, DEFAULT_LOGS_COLOR_MODE, DEFAULT_LOGS_LOG_LEVEL, LOG_CONTEXT,
};
use serde_json::Value;
use services::scripts;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use strum::IntoEnumIterator;

use clap::{ArgAction, Parser, Subcommand};
pub mod services {
    pub mod cli;
    pub mod code;
    pub mod comparators;
    pub mod references;
    pub mod scripts;
    pub mod visitors;
}

pub mod enums {

    pub mod common;
}

pub mod holders {
    pub mod context;
}

pub mod structs {
    pub mod common;
}
pub mod traits {

    pub mod common;
}
fn parse_parameters_val(value: &str) -> Result<Value> {
    Ok(serde_json::from_str(value)?)
}

#[derive(Parser)]
#[command(version, about="OpenAPI v3 translator", long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "TARGET",
        help = "Name of dir with translation scripts and tests"
    )]
    pub target: String,

    #[arg(short='p', long, value_parser = parse_parameters_val, num_args = 1, help = "Parameters for target Lua scripts are simply JSON of arbitrary structure, which will be converted into a Lua table and passed to the scripts as a global parameter named targetParameters. These parameters will replace the parameters passed in the OpenAPI spec as x-ot-target-parameters")]
    pub target_parameters: Option<Value>,
    #[arg(
        short,
        long,
        value_name = "RESOURCES",
        help = "Base dir for all translator resources",
        default_value = "resources"
    )]
    pub resources: PathBuf,

    #[command(subcommand, help = "Action to execution")]
    pub command: Commands,
}

impl Cli {
    pub fn get_tests_dir(&self) -> PathBuf {
        self.get_scripts_dir().join("tests")
    }

    pub fn get_scripts_dir(&self) -> PathBuf {
        self.resources.join("scripts").join(&self.target)
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Test {
        #[arg(
            short,
            action=ArgAction::Append,
            num_args = 1,
            long,
            help = "Optional test name to run (option can be set multiple times)"
        )]
        names: Option<Vec<String>>,
    },
    Translate {
        #[arg(
            short,
            long,
            value_name = "OPENAPI",
            help = "OpenAPI spec to translation"
        )]
        spec: PathBuf,
        #[arg(
            short,
            long,
            value_name = "OUTPATH",
            help = "Path to write output files"
        )]
        out: PathBuf,
        #[arg(short, long, help = "Clean OUTPATH dir before write translated files")]
        clean: bool,
        #[arg(
            short,
            long,
            value_name = "EXPECTED",
            help = "Compare the files in the EXPECTED directory with those in OUTPATH. If differences are found, exit with code 1 and display the diff"
        )]
        expected: Option<PathBuf>,
        test_name: Option<String>,
    },
}

pub fn init_logger() {
    env_logger::Builder::from_env(
        Env::default()
            .default_filter_or(DEFAULT_LOGS_LOG_LEVEL)
            .default_write_style_or(DEFAULT_LOGS_COLOR_MODE),
    )
    .format(move |buf, record| {
        let dynamic_value = LOG_CONTEXT.lock().unwrap();

        if dynamic_value.is_empty() {
            writeln!(
                buf,
                "\n{}{} {}:{} {}{}:\n\n{}",
                Color::Green.paint("["),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                Color::Blue.paint(record.level().to_string()),
                record.file().unwrap_or("unknown source"),
                record.line().unwrap_or(u32::MAX),
                Color::Green.paint("]"),
                record.args()
            )
        } else {
            writeln!(
                buf,
                "\n{}{} {} {}:{} {}{}:\n\n{}",
                Color::Green.paint("["),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                Color::Blue.paint(record.level().to_string()),
                record.file().unwrap_or("unknown source"),
                record.line().unwrap_or(u32::MAX),
                *dynamic_value,
                Color::Green.paint("]"),
                record.args()
            )
        }
    })
    .init();
}

pub fn check_scripts() -> Result<()> {
    let mut scripts_files: HashMap<String, String> = HashMap::new();
    let lua_vm = get_lua_vm();

    for variant in Script::iter() {
        let script_relative_path: &str = (&variant).into();
        let script_path = CLI
            .get_scripts_dir()
            .join(format!("{}.lua", script_relative_path));

        if let Some(old_value) = scripts_files.insert(
            script_path
                .file_name()
                .expect("Unknown script filename")
                .to_str()
                .expect("Script filename not found")
                .to_owned(),
            script_path
                .to_str()
                .expect("Script path conversion error")
                .to_owned(),
        ) {
            return Err(anyhow!(
                "Duplicate script filenames first [{}] second [{:?}]",
                old_value,
                &script_path
            ));
        }
        scripts::get_lua_function(&variant, &lua_vm).context("Script checking error")?;
    }
    Ok(())
}
