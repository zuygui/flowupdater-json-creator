use reqwest::header::HeaderMap;

use crate::errors::CreatorError;

mod minecraft;
mod modloader;
pub(crate) mod mods;

pub static CURSE_API_URL: &str = "https://api.curseforge.com/v1/";

pub struct CurseApi {
    http_client: reqwest::Client,
}

impl CurseApi {
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
