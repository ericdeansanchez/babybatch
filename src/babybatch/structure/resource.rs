//! Resource structure for babybatch.
use crate::structure::BabyBatchResponse;
use crate::util::errors::*;

/// Primary structure representing a _resource_––where a resource is
/// defined by an api key, a country code, and a page token.
#[derive(Debug)]
pub struct Resource {
    api_key: String,
    country_code: String,
    page_token: String,
}

/// Default implementation for a given `Resource`.
impl Default for Resource {
    fn default() -> Self {
        Resource {
            api_key: String::new(),
            country_code: String::new(),
            page_token: String::new(),
        }
    }
}

impl Resource {
    pub fn new(api_key: &str) -> Self {
        Resource {
            api_key: api_key.to_owned(),
            ..Default::default()
        }
    }

    pub fn country_code(mut self, c: &str) -> Self {
        self.country_code = c.to_owned();
        self
    }

    pub fn page_token(mut self, t: &str) -> Self {
        self.page_token = t.to_owned();
        self
    }

    pub fn set_token(&mut self, t: &str) {
        self.page_token = t.to_owned();
    }

    pub fn request(&self) -> Result<BabyBatchResponse> {
        let mut response = reqwest::get(&Resource::_url(&self))?;
        let response: serde_json::Value = response.json()?;
        let value: &serde_json::Value = &response["nextPageToken"];

        if let Some(page_token) = value.as_str() {
            let page_token = format!("&pageToken={}&", page_token);
            Ok(BabyBatchResponse {
                body: response,
                page_token,
            })
        } else {
            Err(BabyBatchError::NoNextPageToken)
        }
    }

    fn _url(resource: &Resource) -> String {
        const URL: &str = "https://www.googleapis.com/youtube/v3/videos?part";

        if !resource.page_token.is_empty() {
            format!(
        "{URL}=id,statistics,snippet{PAGE_TOKEN}chart=mostPopular&regionCode={COUNTRY_CODE}&maxResults=50&key={API_KEY}",
        URL = URL,
        PAGE_TOKEN = resource.page_token,
        COUNTRY_CODE = resource.country_code,
        API_KEY = resource.api_key)
        } else {
            "".to_owned()
        }
    }
}
