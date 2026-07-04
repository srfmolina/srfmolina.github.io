//! Data layer — Krocy's `data` Gradle module. api → datasource → repository.

pub mod api;
pub mod datasource;
pub mod entity;
pub mod mapper;
pub mod repository;

use crate::data::api::links_api::LinksApiImpl;
use crate::data::datasource::links::links_data_source::LinksDataSourceImpl;
use crate::data::repository::links_repository_impl::LinksRepositoryImpl;
use crate::domain::usecase::links::get_links_use_case::GetLinksUseCase;

/// Concrete, nameable type of the fully-wired GetLinks use case — needed so it
/// can live in Dioxus context (an `impl Trait` cannot be named or stored).
pub type GetLinks =
    GetLinksUseCase<LinksRepositoryImpl<LinksDataSourceImpl<LinksApiImpl>>>;

/// Composition root (Krocy's Koin `DataModule`): wires
/// api → datasource → repository → usecase against the live URL.
pub fn provide_get_links_use_case() -> GetLinks {
    let api = LinksApiImpl::default_url();
    let data_source = LinksDataSourceImpl::new(api);
    let repository = LinksRepositoryImpl::new(data_source);
    GetLinksUseCase::new(repository)
}

/// The app's use cases, instantiated once at the top level and provided
/// globally via Dioxus context. Use cases only — repositories and datasources
/// stay encapsulated here. Extensible: add more use-case fields over time.
#[derive(Clone)]
pub struct UseCases {
    pub get_links: GetLinks,
}

impl UseCases {
    pub fn new() -> Self {
        Self { get_links: provide_get_links_use_case() }
    }
}

impl Default for UseCases {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_cases_construct_and_clone() {
        // Building the wired chain and cloning it must not panic and must
        // compile — which transitively requires Clone on every layer.
        let use_cases = UseCases::new();
        let _cloned = use_cases.clone();
    }
}
