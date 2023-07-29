use reqwest::header::HeaderMap;

use crate::errors::CreatorError;

pub mod minecraft;
pub mod modloader;
pub mod mods;

/// The base URL for the CurseForge API.
pub static CURSE_API_URL: &str = "https://api.curseforge.com/v1/";

/// The CurseForge API wrapper.
pub struct CurseApi {
    http_client: reqwest::Client,
}

impl CurseApi {
    /// Creates a new CurseForge API wrapper.
    /// 
    /// # Arguments
    /// 
    /// * `api_token` - The API token to use for requests.
    /// 
    /// # Returns
    /// 
    /// A new CurseForge API wrapper.
    /// 
    /// # Errors
    /// 
    /// If the API token is invalid, or if the HTTP client cannot be created.
    pub fn new<S>(api_token: S) -> Result<Self, CreatorError>
    where
        S: AsRef<str>,
    {
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", api_token.as_ref().parse().unwrap());

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { http_client })
    }
}
