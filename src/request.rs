use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::custom_errors::CustomError;

pub async fn fetch_data(url: String, semaphore: Arc<Semaphore>, client: reqwest::Client) -> Result<RequestResponse, CustomError> {
    let _permit = semaphore.acquire().
        await.
        map_err(|_e| CustomError::UnexpectedError)?;

    let resp = client.get(url).send().await.
        map_err(|_e| CustomError::UnexpectedError)?;

    if !resp.status().is_success() {
        return Ok(
            RequestResponse::HttpError{code: resp.status().as_u16()}
        );
    }

    let body = resp.text().await.
        map_err(|_e| CustomError::UnexpectedError)?;

    Ok(
        RequestResponse::Ok{title: body}
    )
}

#[derive(Debug)]
pub enum RequestResponse {
    Ok{title: String},
    HttpError{code: u16},

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
            then.status(200).body("hello world");
        });

        let url = server.url("/ok");
        let semaphore = Arc::new(Semaphore::new(1));
        let client = reqwest::Client::new();

        let result = fetch_data(url, semaphore, client).await;

        match result {
            Ok(RequestResponse::Ok { title }) => assert_eq!(title, "hello world"),
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
            Ok(RequestResponse::HttpError { code }) => assert_eq!(code, 404),
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