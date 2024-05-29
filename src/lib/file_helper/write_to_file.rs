use crate::lib::error::FileHeleperError;
use anyhow::Context;

use std::fs;

pub async fn write_to_file(path: &str, file_name: &str, content: &str) -> anyhow::Result<()> {
    let full_path = format!("{}/{}", path.to_owned(), file_name.to_owned());

    if fs::metadata(&full_path).is_ok() {
        return Err(anyhow::anyhow!(FileHeleperError::FileAlreadyExists));
    }

    tracing::info!("Creating file: {}", full_path);
    let _ = fs::create_dir_all(path).context("Failed to create directory");
    let _ = fs::write(&full_path, content).context("Failed to create file");
    tracing::info!("File created: {}", full_path);

    Ok(())
}

mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_write_to_file() {
    //     let path = "/Users/kod-x/project/rust_project/flutter_gen/bloc";
    //     let file_name = "test_bloc.dart";
    //     let context = "  ";
    //     let result: Result<(), anyhow::Error> = write_to_file(path, file_name, context).await;
    //
    //     assert!(result.is_ok());
    // }
}
