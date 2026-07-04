//! Projects screen ViewModel — the Dioxus mirror of Krocy's `XViewModel.kt`.
//!
//! On `Init` it loads the link catalog through `GetLinksUseCase`, pulled from
//! the app-global `UseCases` context, and reduces it to view-ready
//! `ProjectCard`s. The DTO→card flattening (`cards_from_links`) is pure and is
//! unit-tested on the host; the fetch and rendering need the wasm runtime.

use dioxus::prelude::*;

use crate::data::UseCases;
use crate::domain::model::links::{Links, ProjectLink};
use crate::ui::base::*;
use crate::view_model;

// --- PRESENTATION MODEL ---

/// Which forge a project is hosted on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Host {
    GitHub,
    Codeberg,
}

impl Host {
    /// The forge's display name — a proper noun, not localized.
    pub fn label(self) -> &'static str {
        match self {
            Host::GitHub => "GitHub",
            Host::Codeberg => "Codeberg",
        }
    }
}

/// A single repository card, ready to render.
#[derive(Clone, PartialEq)]
pub struct ProjectCard {
    pub host: Host,
    pub name: String,
    pub language: Option<String>,
    pub url: String,
    pub description: String,
}

/// Last non-empty path segment of a repo URL, e.g.
/// `https://github.com/srfmolina/mod-comp` → `mod-comp`.
fn name_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or("")
        .to_string()
}

fn card(host: Host, link: ProjectLink) -> ProjectCard {
    ProjectCard {
        host,
        name: name_from_url(&link.url),
        language: link.language,
        url: link.url,
        description: link.description,
    }
}

/// Flatten the catalog into cards: GitHub first, then Codeberg. Contacts are
/// intentionally unused on this screen.
pub fn cards_from_links(links: Links) -> Vec<ProjectCard> {
    links
        .github
        .into_iter()
        .map(|link| card(Host::GitHub, link))
        .chain(links.codeberg.into_iter().map(|link| card(Host::Codeberg, link)))
        .collect()
}

// --- STATE ---

/// The screen's load status.
#[derive(Clone, PartialEq)]
pub enum ProjectsStatus {
    Loading,
    Loaded(Vec<ProjectCard>),
    Error,
}

#[derive(Clone, PartialEq)]
pub struct ProjectsState {
    pub status: ProjectsStatus,
}
impl UiState for ProjectsState {}

// --- EVENT ---

pub enum ProjectsEvent {
    /// First render: load the catalog through the use case.
    Init,
}
impl UiEvent for ProjectsEvent {}

// --- EFFECT ---

pub enum ProjectsEffect {}
impl UiEffect for ProjectsEffect {}

// --- VIEWMODEL ---

view_model!(pub ProjectsViewModel<ProjectsState, ProjectsEffect>);

impl BaseViewModel for ProjectsViewModel {
    type Event = ProjectsEvent;

    fn create_initial_state() -> ProjectsState {
        ProjectsState { status: ProjectsStatus::Loading }
    }

    async fn handle_event(self, event: ProjectsEvent) {
        match event {
            // Initial state is already Loading (see create_initial_state).
            ProjectsEvent::Init => {
                let result = match try_consume_context::<UseCases>() {
                    Some(use_cases) => use_cases
                        .get_links
                        .invoke()
                        .await
                        .map(cards_from_links)
                        .map_err(|_| ()),
                    None => Err(()),
                };
                self.set_state(move |_| ProjectsState {
                    status: match &result {
                        Ok(cards) => ProjectsStatus::Loaded(cards.clone()),
                        Err(()) => ProjectsStatus::Error,
                    },
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::links::{Contact, Links, ProjectLink};

    fn link(url: &str, lang: Option<&str>) -> ProjectLink {
        ProjectLink {
            url: url.to_string(),
            description: "d".to_string(),
            language: lang.map(str::to_string),
        }
    }

    #[test]
    fn name_is_the_last_url_segment() {
        assert_eq!(name_from_url("https://github.com/srfmolina/mod-comp"), "mod-comp");
    }

    #[test]
    fn name_tolerates_a_trailing_slash() {
        assert_eq!(name_from_url("https://codeberg.org/srfmolina/pages/"), "pages");
    }

    #[test]
    fn cards_are_github_then_codeberg_with_host_tagged() {
        let links = Links {
            github: vec![link("https://github.com/u/a", Some("Rust"))],
            codeberg: vec![link("https://codeberg.org/u/b", None)],
            contacts: vec![Contact { name: "linkedin".into(), url: "x".into() }],
        };

        let cards = cards_from_links(links);

        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].host, Host::GitHub);
        assert_eq!(cards[0].name, "a");
        assert_eq!(cards[0].language.as_deref(), Some("Rust"));
        assert_eq!(cards[1].host, Host::Codeberg);
        assert_eq!(cards[1].name, "b");
        assert_eq!(cards[1].language, None);
    }

    #[test]
    fn empty_catalog_yields_no_cards() {
        let links = Links { github: vec![], codeberg: vec![], contacts: vec![] };
        assert!(cards_from_links(links).is_empty());
    }
}
