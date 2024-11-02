use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{Context, Result};
use log::{info, warn};

use crate::{enums::common::WriteMode, structs::common::Code};

pub fn save_code(out_path: &Path, code: Vec<Code>) -> Result<()> {
    code.iter().try_for_each(|it| {
        let code_path = out_path.join(&it.file);
        // Create the directories recursively if they don't exist
        if let Some(parent) = code_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
        modify_file(&code_path, &it.code, &it.mode).with_context(|| {
            format!(
                "Could not write code with mode [{:?}] to [{:?}]",
                &it.mode, &code_path
            )
        })
    })
}

fn modify_file(file_path: &Path, text: &Option<String>, mode: &WriteMode) -> Result<()> {
    let file_exists = File::open(file_path).is_ok();

    match mode {
        WriteMode::Prepend => {
            if let Some(text) = text {
                if file_exists {
                    let mut existing_file = OpenOptions::new().read(true).open(file_path)?;

                    let mut contents = String::new();
                    existing_file.read_to_string(&mut contents)?;

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(file_path)?;

                    file.write_all(text.as_bytes())?;
                    file.write_all(contents.as_bytes())?;
                } else {
                    let mut file = File::create(file_path)?;
                    file.write_all(text.as_bytes())?;
                }
            } else {
                warn!("Empty text prepend to file [{:?}]", file_path)
            }
        }
        WriteMode::Append => {
            if let Some(text) = text {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_path)?;

                file.write_all(text.as_bytes())?;
            } else {
                warn!("Empty text prepend to file [{:?}]", file_path)
            }
        }
        WriteMode::Remove => {
            if file_path.exists() {
                match fs::remove_file(file_path) {
                    Ok(_) => warn!("Removed file [{:?}]", file_path),
                    Err(e) => warn!("File removing error: [{:?}]", e),
                }
            } else {
                info!("File for removing is not exists [{:?}]", file_path);
            }
        }
    }

    Ok(())
}
