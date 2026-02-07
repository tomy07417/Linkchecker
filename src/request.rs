use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::custom_errors::CustomError;
use crate::scraper::extract_title;

/// Fetches a URL while respecting the provided concurrency limit.
///
/// The request is executed with the given `client` and guarded by the
/// `semaphore` to cap concurrent requests. If the response status is not
/// successful, the function returns `RequestResponse::HttpError` with the
/// status code. On success, it returns `RequestResponse::Ok` with the body.
///
/// # Errors
///
/// Returns `CustomError::UnexpectedError` when acquiring a permit, sending the
/// request, or reading the response body fails.
pub async fn fetch_data(
    url: String,
    semaphore: Arc<Semaphore>,
    client: reqwest::Client,
) -> Result<RequestResponse, CustomError> {
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|_e| CustomError::UnexpectedError)?;

    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|_e| CustomError::UnexpectedError)?;

    if !resp.status().is_success() {
        let reason = resp
            .status()
            .canonical_reason()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("HTTP {}", resp.status().as_u16()));

        return Ok(RequestResponse::HttpError { reason });
    }

    let body = resp
        .text()
        .await
        .map_err(|_e| CustomError::UnexpectedError)?;

    let title = extract_title(&body).unwrap_or_else(|| "No title found".to_string());

    Ok(RequestResponse::Ok { title })
}

/// Result of an HTTP request performed by `fetch_data`.
#[derive(Debug)]
pub enum RequestResponse {
    Ok { title: String },
    HttpError { reason: String },
}

impl std::fmt::Display for RequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestResponse::Ok { title } => write!(f, "{}", title),
            RequestResponse::HttpError { reason } => write!(f, "{}", reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method::GET, MockServer};

    #[tokio::test]
    async fn fetch_data_returns_body_on_success() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/ok");
            then.status(200)
                .body("<!doctype html><html><head><title>Hello Title</title></head><body>hello world</body></html>");
        });

        let url = server.url("/ok");
        let semaphore = Arc::new(Semaphore::new(1));
        let client = reqwest::Client::new();

        let result = fetch_data(url, semaphore, client).await;

        match result {
            Ok(RequestResponse::Ok { title }) => assert_eq!(title, "Hello Title"),
            _ => panic!("expected Ok response"),
        }
    }

    #[tokio::test]
    async fn fetch_data_returns_http_error_on_non_success_status() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/missing");
            then.status(404);
        });

        let url = server.url("/missing");
        let semaphore = Arc::new(Semaphore::new(1));
        let client = reqwest::Client::new();

        let result = fetch_data(url, semaphore, client).await;

        match result {
            Ok(RequestResponse::HttpError { reason }) => assert_eq!(reason, "Not Found"),
            _ => panic!("expected HttpError response"),
        }
    }

    #[tokio::test]
    async fn fetch_data_returns_unexpected_error_on_invalid_url() {
        let url = "http:://bad-url".to_string();
        let semaphore = Arc::new(Semaphore::new(1));
        let client = reqwest::Client::new();

        let result = fetch_data(url, semaphore, client).await;

        match result {
            Err(CustomError::UnexpectedError) => {}
            _ => panic!("expected UnexpectedError"),
        }
    }
}
