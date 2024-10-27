use anyhow::{anyhow, Result};
use diffy::create_patch;
use log::error;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn assert_diff(actual_path: &Path, expected_path: &Path) -> Result<()> {
    let mut test_results: Vec<Result<()>> = Vec::new();
    for entry in fs::read_dir(actual_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let actual_file = entry.path();
        let expected_file = expected_path.join(&file_name);

        if expected_file.exists() && expected_file.is_file() {
            test_results.push(compare_and_save_diff(
                &actual_file,
                &expected_file,
                actual_path,
            ));
        }
    }

    test_results.iter().for_each(|it| {
        if let Err(err) = it {
            error!("{}", err);
        }
    });

    let total_tests_count = test_results.len();
    let failed_tests_count = test_results.iter().filter(|it| it.is_err()).count();

    if test_results.into_iter().any(|it| it.is_err()) {
        return Err(anyhow!(
            "Found [{}] failed tests from [{}]",
            failed_tests_count,
            total_tests_count
        ));
    }

    Ok(())
}

fn compare_and_save_diff(actual_path: &Path, expected_path: &Path, patch_dir: &Path) -> Result<()> {
    let actual_content = fs::read_to_string(actual_path)?;
    let expected_content = fs::read_to_string(expected_path)?;

    let patch = create_patch(&expected_content, &actual_content);

    let patch_str = format!("{}", patch);

    if patch_str != "--- original\n+++ modified\n" {
        let patch_file_name = format!(
            "{}.patch",
            actual_path
                .file_stem()
                .ok_or_else(|| anyhow!(
                    "File name without extension not found for [{:?}]",
                    actual_path
                ))?
                .to_str()
                .ok_or_else(|| anyhow!(
                    "File name conversion to string error [{:?}]",
                    actual_path
                ))?
        );
        let patch_file_path = patch_dir.join(patch_file_name);

        let mut patch_file = File::create(&patch_file_path)?;
        patch_file.write_all(patch_str.as_bytes())?;
        return Err(anyhow!(
            "\nFailed test with expected value\n[{:?}]\nactual value\n[{:?}]\ndiff saved to\n[{:?}]",
            expected_path,
            actual_path,
            patch_file_path
        ));
    }
    Ok(())
}
