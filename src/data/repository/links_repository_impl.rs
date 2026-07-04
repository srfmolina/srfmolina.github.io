//! `LinksRepository` implementation: datasource → mapper → domain.
//! Krocy's `data.repository.impl.*RepositoryImpl` (folder flattened — `impl` is reserved).

use crate::data::datasource::links::links_data_source::LinksDataSource;
use crate::data::mapper::links_mapper::map_links;
use crate::domain::error::LinkError;
use crate::domain::model::links::Links;
use crate::domain::repository::links_repository::LinksRepository;

/// Fetches via the datasource and maps the DTO onto the domain aggregate.
#[derive(Clone)]
pub struct LinksRepositoryImpl<D: LinksDataSource> {
    data_source: D,
}

impl<D: LinksDataSource> LinksRepositoryImpl<D> {
    pub fn new(data_source: D) -> Self {
        Self { data_source }
    }
}

impl<D: LinksDataSource> LinksRepository for LinksRepositoryImpl<D> {
    async fn get_links(&self) -> Result<Links, LinkError> {
        let dto = self.data_source.get_links().await?;
        Ok(map_links(dto))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::entity::links::{LinkDto, LinksDto};

    struct FakeDataSource(Result<LinksDto, LinkError>);

    impl LinksDataSource for FakeDataSource {
        async fn get_links(&self) -> Result<LinksDto, LinkError> {
            self.0.clone()
        }
    }

    #[test]
    fn maps_datasource_dto_into_domain() {
        let dto = LinksDto {
            github: vec![LinkDto { link: "g".into(), description: "d".into(), language: None }],
            codeberg: vec![],
            extra: std::collections::BTreeMap::new(),
        };
        let repo = LinksRepositoryImpl::new(FakeDataSource(Ok(dto)));

        let result = futures::executor::block_on(repo.get_links());

        let links = result.expect("ok");
        assert_eq!(links.github.len(), 1);
        assert_eq!(links.github[0].url, "g");
    }

    #[test]
    fn propagates_datasource_error() {
        let repo = LinksRepositoryImpl::new(FakeDataSource(Err(LinkError::Network("boom".into()))));

        let result = futures::executor::block_on(repo.get_links());

        assert_eq!(result, Err(LinkError::Network("boom".into())));
    }
}
