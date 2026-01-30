use crate::custom_errors::CustomError;
use regex::Regex;
use std::{collections::HashSet, fs, path::Path};

/// Extracts unique URLs from a text file.
///
/// The function reads the file at `path`, finds all occurrences of URLs that
/// match the pattern `http://` or `https://`, and returns them in the order
/// they appear, skipping duplicates.
///
/// # Errors
///
/// Returns `CustomError::FileNotExist` if the file does not exist and
/// `CustomError::FileReadError` if the file cannot be read.
pub fn extract_urls_from_input(path: &Path) -> Result<Vec<String>, CustomError> {
    let list_of_urls: String = read_file(path)?;

    let re = Regex::new(r"https?://[^\s\)\]]+").unwrap();

    let mut urls_seen = HashSet::new();
    let mut urls = Vec::new();

    for cap in re.find_iter(&list_of_urls) {
        let url = cap.as_str().to_string();

        if urls_seen.insert(url.clone()) {
            urls.push(url);
        }
    }

    Ok(urls)
}

/// Reads a file and returns its contents as a `String`.
///
/// # Errors
///
/// Returns `CustomError::FileNotExist` if the file does not exist and
/// `CustomError::FileReadError` if the file cannot be read.
fn read_file(path: &Path) -> Result<String, CustomError> {
    if !fs::exists(path).map_err(|_e| {
        CustomError::UnexpectedError
    })? {
        return Err(CustomError::FileNotExist(
            path.to_string_lossy().to_string(),
        ));
    }

    fs::read_to_string(path)
        .map_err(|_e| CustomError::FileReadError(path.to_string_lossy().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("linkchecker_{}_{}", name, nanos));
        path
    }

    fn write_temp_file(name: &str, contents: &str) -> PathBuf {
        let path = temp_path(name);
        fs::write(&path, contents).expect("should write temp file");
        path
    }

    #[test]
    fn extract_urls_from_input_finds_unique_links_in_order() {
        let input = r#"
        - https://www.rust-lang.org
        - [Search](https://www.google.com)
        - https://www.rust-lang.org
        - text without url
        - https://this-link-does-not-exist.xyz
        "#;
        let path = write_temp_file("extract_basic", input);

        let urls = extract_urls_from_input(&path).expect("should extract urls");

        // The current regex includes the closing ')' for Markdown links.
        assert_eq!(
            urls,
            vec![
                "https://www.rust-lang.org".to_string(),
                "https://www.google.com".to_string(),
                "https://this-link-does-not-exist.xyz".to_string(),
            ],
            "should return unique urls preserving first-seen order"
        );

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn extract_urls_from_input_empty_file_returns_empty_list() {
        let path = write_temp_file("extract_empty", "");

        let urls = extract_urls_from_input(&path).expect("should extract urls");
        assert!(urls.is_empty(), "empty input should yield no urls");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn extract_urls_from_input_text_without_links_returns_empty_list() {
        let path = write_temp_file("extract_no_urls", "no urls here\njust text");

        let urls = extract_urls_from_input(&path).expect("should extract urls");
        assert!(urls.is_empty(), "text-only input should yield no urls");

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn extract_urls_from_input_missing_file_reports_not_found() {
        let path = temp_path("extract_missing");

        let result = extract_urls_from_input(&path);
        match result {
            Err(CustomError::FileNotExist(p)) => {
                assert!(
                    p.contains(&*path.to_string_lossy()),
                    "error should include the missing path"
                );
            }
            _ => panic!("expected FileNotExist error"),
        }
    }

    #[test]
    #[cfg(unix)]
    fn extract_urls_from_input_unreadable_file_reports_read_error() {
        let path = write_temp_file("extract_no_perm", "https://example.com");

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o000);
        fs::set_permissions(&path, perms).expect("remove permissions");

        let result = extract_urls_from_input(&path);
        match result {
            Err(CustomError::FileReadError(p)) => {
                assert!(
                    p.contains(&*path.to_string_lossy()),
                    "error should include the unreadable path"
                );
            }
            _ => panic!("expected FileReadError error"),
        }

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o644);
        let _ = fs::set_permissions(&path, perms);
        let _ = fs::remove_file(&path);
    }
}
