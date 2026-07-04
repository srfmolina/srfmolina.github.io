//! Entry point for reading the link catalog (Krocy's `domain.usecase`).
//! Kept simple — no `ResultUseCase` base; `invoke()` forwards to the repository.

use crate::domain::error::LinkError;
use crate::domain::model::links::Links;
use crate::domain::repository::links_repository::LinksRepository;

/// Reads the full link catalog through the repository.
#[derive(Clone)]
pub struct GetLinksUseCase<R: LinksRepository> {
    repository: R,
}

impl<R: LinksRepository> GetLinksUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn invoke(&self) -> Result<Links, LinkError> {
        self.repository.get_links().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeRepository(Result<Links, LinkError>);

    impl LinksRepository for FakeRepository {
        async fn get_links(&self) -> Result<Links, LinkError> {
            self.0.clone()
        }
    }

    #[test]
    fn invoke_forwards_repository_success() {
        let links = Links::default();
        let use_case = GetLinksUseCase::new(FakeRepository(Ok(links.clone())));

        let result = futures::executor::block_on(use_case.invoke());

        assert_eq!(result, Ok(links));
    }

    #[test]
    fn invoke_forwards_repository_error() {
        let use_case = GetLinksUseCase::new(FakeRepository(Err(LinkError::Decode("bad".into()))));

        let result = futures::executor::block_on(use_case.invoke());

        assert_eq!(result, Err(LinkError::Decode("bad".into())));
    }
}
