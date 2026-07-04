//! Remote datasource for the links document (Krocy's `data.datasource`).
//! The `Result` seam that tests fake.

use crate::data::entity::links::LinksDto;
use crate::domain::error::LinkError;

#[allow(async_fn_in_trait)]
pub trait LinksDataSource {
    async fn get_links(&self) -> Result<LinksDto, LinkError>;
}

use crate::data::api::links_api::LinksApi;

/// Wraps the api as the repository-facing `Result` seam.
#[derive(Clone)]
pub struct LinksDataSourceImpl<A: LinksApi> {
    api: A,
}

impl<A: LinksApi> LinksDataSourceImpl<A> {
    pub fn new(api: A) -> Self {
        Self { api }
    }
}

impl<A: LinksApi> LinksDataSource for LinksDataSourceImpl<A> {
    async fn get_links(&self) -> Result<LinksDto, LinkError> {
        self.api.fetch().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::api::links_api::LinksApi;
    use crate::data::entity::links::LinksDto;

    struct FakeApi(Result<LinksDto, LinkError>);

    impl LinksApi for FakeApi {
        async fn fetch(&self) -> Result<LinksDto, LinkError> {
            self.0.clone()
        }
    }

    #[test]
    fn datasource_forwards_api_result() {
        let ds = LinksDataSourceImpl::new(FakeApi(Ok(LinksDto {
            github: vec![],
            codeberg: vec![],
            extra: std::collections::BTreeMap::new(),
        })));

        let result = futures::executor::block_on(ds.get_links());

        assert!(result.is_ok());
    }

    #[test]
    fn datasource_forwards_api_error() {
        let ds = LinksDataSourceImpl::new(FakeApi(Err(LinkError::Network("x".into()))));

        let result = futures::executor::block_on(ds.get_links());

        assert_eq!(result, Err(LinkError::Network("x".into())));
    }
}
