//! Repository contract (Krocy's `domain.repository`). Implemented in the data layer.

use crate::domain::error::LinkError;
use crate::domain::model::links::Links;

#[allow(async_fn_in_trait)]
pub trait LinksRepository {
    async fn get_links(&self) -> Result<Links, LinkError>;
}
