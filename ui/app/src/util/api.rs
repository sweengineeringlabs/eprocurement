//! API client utilities

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use serde::{de::DeserializeOwned, Serialize};

/// Get API base URL from environment
pub fn api_base() -> String {
    // In production, this would come from environment config
    "/eprocurement/api/v1".to_string()
}

/// HTTP methods
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }
    }
}

/// API error type
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error {}: {}", self.status, self.message)
    }
}

/// Make an API request
pub async fn request<T: DeserializeOwned>(
    method: HttpMethod,
    endpoint: &str,
    body: Option<String>,
) -> Result<T, ApiError> {
    let url = format!("{}{}", api_base(), endpoint);

    let mut opts = RequestInit::new();
    opts.method(method.as_str());

    if let Some(b) = body {
        opts.body(Some(&JsValue::from_str(&b)));
    }

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| ApiError {
            status: 0,
            message: "Failed to create request".to_string(),
        })?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .ok();

    let window = web_sys::window().unwrap();
    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| ApiError {
            status: 0,
            message: "Network error".to_string(),
        })?;

    let response: Response = response_value.dyn_into().unwrap();
    let status = response.status();

    if !response.ok() {
        return Err(ApiError {
            status,
            message: format!("Request failed with status {}", status),
        });
    }

    let json = JsFuture::from(response.json().unwrap())
        .await
        .map_err(|_| ApiError {
            status,
            message: "Failed to parse response".to_string(),
        })?;

    serde_wasm_bindgen::from_value(json).map_err(|e| ApiError {
        status,
        message: format!("JSON parse error: {}", e),
    })
}

/// GET request
pub async fn get<T: DeserializeOwned>(endpoint: &str) -> Result<T, ApiError> {
    request(HttpMethod::Get, endpoint, None).await
}

/// POST request with JSON body
pub async fn post<T: DeserializeOwned, B: Serialize>(
    endpoint: &str,
    body: &B,
) -> Result<T, ApiError> {
    let json = serde_json::to_string(body).map_err(|e| ApiError {
        status: 0,
        message: format!("Serialization error: {}", e),
    })?;
    request(HttpMethod::Post, endpoint, Some(json)).await
}

/// PUT request with JSON body
pub async fn put<T: DeserializeOwned, B: Serialize>(
    endpoint: &str,
    body: &B,
) -> Result<T, ApiError> {
    let json = serde_json::to_string(body).map_err(|e| ApiError {
        status: 0,
        message: format!("Serialization error: {}", e),
    })?;
    request(HttpMethod::Put, endpoint, Some(json)).await
}

/// DELETE request
pub async fn delete<T: DeserializeOwned>(endpoint: &str) -> Result<T, ApiError> {
    request(HttpMethod::Delete, endpoint, None).await
}
