//! The api layer — implemented here (Krocy delegates this to the `grocy-client`
//! module). A thin reqwest GET + serde decode returning the raw DTO.

use crate::data::entity::links::LinksDto;
use crate::domain::error::LinkError;

/// Public raw URL of the links document.
pub const LINKS_URL: &str =
    "https://raw.githubusercontent.com/srfmolina/links/links/index.json";

#[allow(async_fn_in_trait)]
pub trait LinksApi {
    async fn fetch(&self) -> Result<LinksDto, LinkError>;
}

/// reqwest-backed api. On wasm this uses the browser fetch backend.
#[derive(Clone)]
pub struct LinksApiImpl {
    url: String,
}

impl LinksApiImpl {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub fn default_url() -> Self {
        Self::new(LINKS_URL)
    }
}

impl LinksApi for LinksApiImpl {
    async fn fetch(&self) -> Result<LinksDto, LinkError> {
        let response = reqwest::get(&self.url)
            .await
            .map_err(|e| LinkError::Network(e.to_string()))?
            .error_for_status()
            .map_err(|e| LinkError::Network(e.to_string()))?;

        response
            .json::<LinksDto>()
            .await
            .map_err(|e| LinkError::Decode(e.to_string()))
    }
}
