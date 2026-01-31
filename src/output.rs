use std::{fs::File, path::Path};
use std::io::Write;

use crate::{custom_errors::CustomError, request::RequestResponse};

pub fn write_output_file(
    path: &Path, content: Vec<(String, RequestResponse)>
) -> Result<(), CustomError> {

    let mut file = File::create(path).
        map_err(|_| CustomError::FileWriteError(format!("Error writing file: {}", path.display())))?;

    for (url, res) in content {
        write!(
            file,
            "[{}] ({})\n",
            res,
            url
        ).map_err(|_| CustomError::FileWriteError(format!("Error writing to file: {}", path.display())))?;
    }

    Ok(())
}