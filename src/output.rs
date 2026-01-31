use std::io::Write;
use std::{fs::File, path::Path};

use crate::{custom_errors::CustomError, request::RequestResponse};

/// Writes the link check results to a Markdown file.
///
/// Each entry is written as `[title_or_reason] (url)` on its own line, where the
/// `RequestResponse` is formatted via its `Display` implementation.
///
/// # Errors
///
/// Returns `CustomError::FileWriteError` if the file cannot be created or any
/// write fails.
pub fn write_output_file(
    path: &Path,
    content: Vec<(String, RequestResponse)>,
) -> Result<(), CustomError> {
    let mut file = File::create(path).map_err(|_| {
        CustomError::FileWriteError(format!("Error writing file: {}", path.display()))
    })?;

    for (url, res) in content {
        writeln!(file, "[{}] ({})", res, url).map_err(|_| {
            CustomError::FileWriteError(format!("Error writing to file: {}", path.display()))
        })?;
    }

    Ok(())
}
