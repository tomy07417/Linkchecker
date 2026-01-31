use std::{path::Path, sync::Arc};

use crate::custom_errors::CustomError;

mod custom_errors;
mod parser;
mod request;
mod scraper;
mod output;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file_path> <output_file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let urls: Vec<String> = parser::extract_urls_from_input(file_path).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let semaphore = Arc::new(tokio::sync::Semaphore::new(32));
    let client = reqwest::Client::new();

    let mut handles = Vec::new();

    for url in urls {
        let url_cp = url.clone();
        let smp = semaphore.clone();
        let cli = client.clone();

        let handle = tokio::spawn(async move { request::fetch_data(url_cp, smp, cli).await });
        handles.push((url, handle));
    }

    let mut response = Vec::new();

    for (url, handle) in handles {
        let _ = handle
            .await
            .map_err(|_e| CustomError::UnexpectedError)
            .and_then(|res| res)
            .inspect_err(|e| {
                eprintln!("[{}] {}", e, url);
            })
            .map(|res| {
                response.push((url.clone(), res));
            });
    }

    if let Err(e) = output::write_output_file(output_path, response) {
        eprintln!("Error writing output file: {}", e);
        std::process::exit(1);
    }
}
