//! Projects screen — the Dioxus mirror of Krocy's `XScreen.kt`.
//!
//! Renders only the routed `<main>` (the chrome is the router's `Scaffold`).
//! Drives its own `ProjectsViewModel`, fires `Init` on first render to load the
//! catalog through the use case, and renders Loading / the card grid / Error.

use dioxus::prelude::*;

use super::projects_view_model::{
    Host, ProjectCard, ProjectsEvent, ProjectsStatus, ProjectsViewModel,
};
use crate::ui::base::*;
use crate::ui::i18n::{Language, Texts};

/// The Projects screen (main content only).
#[component]
pub fn ProjectsScreen() -> Element {
    let language = use_context::<ReadSignal<Language>>();

    let vm = use_view_model::<ProjectsViewModel>();
    use_hook(|| vm.launch_event(ProjectsEvent::Init));

    let texts = Texts::for_language(*language.read());
    let projects = &texts.projects;
    let status = vm.state().read().status.clone();

    rsx! {
        main { class: "main",
            section { class: "section projects-head",
                div { class: "projects-eyebrow", "{projects.eyebrow}" }
                h1 { class: "section-title", "{projects.title}" }
                p { class: "projects-lead", "{projects.lead}" }
            }

            {
                match status {
                    ProjectsStatus::Loading => rsx! {
                        div { class: "projects-status", "{projects.loading}" }
                    },
                    ProjectsStatus::Error => rsx! {
                        div { class: "projects-status", "{projects.error}" }
                    },
                    ProjectsStatus::Loaded(cards) => rsx! {
                        section { class: "section",
                            div { class: "repo-grid",
                                for c in cards.iter() {
                                    RepoCard { key: "{c.url}", card: c.clone(), view_repo: projects.view_repo }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

/// A single repository card linking out to the repo.
#[component]
fn RepoCard(card: ProjectCard, view_repo: &'static str) -> Element {
    rsx! {
        a {
            class: "repo-card",
            href: "{card.url}",
            target: "_blank",
            rel: "noopener noreferrer",
            div { class: "repo-card-top",
                span { class: "repo-host",
                    span { class: "repo-host-icon", HostIcon { host: card.host } }
                    "{card.host.label()}"
                }
                if let Some(lang) = card.language.as_ref() {
                    span { class: "repo-lang", "{lang}" }
                }
            }
            h3 { class: "repo-name", "{card.name}" }
            p { class: "repo-desc", "{card.description}" }
            div { class: "repo-cta",
                "{view_repo}"
                span { class: "repo-cta-arrow", "→" }
            }
        }
    }
}

/// The forge glyph for a card — GitHub or Codeberg mark.
#[component]
fn HostIcon(host: Host) -> Element {
    match host {
        Host::GitHub => rsx! {
            svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "currentColor",
                path { d: "M12 1.5a10.5 10.5 0 0 0-3.32 20.46c.52.1.71-.23.71-.5v-1.78c-2.92.64-3.54-1.24-3.54-1.24-.48-1.22-1.17-1.54-1.17-1.54-.95-.65.07-.64.07-.64 1.06.07 1.61 1.09 1.61 1.09.94 1.6 2.46 1.14 3.06.87.1-.68.37-1.14.66-1.4-2.33-.27-4.78-1.17-4.78-5.18 0-1.15.41-2.08 1.08-2.82-.11-.27-.47-1.34.1-2.79 0 0 .88-.28 2.88 1.07a9.96 9.96 0 0 1 5.24 0c2-1.35 2.88-1.07 2.88-1.07.57 1.45.21 2.52.1 2.79.67.74 1.08 1.67 1.08 2.82 0 4.02-2.46 4.9-4.8 5.16.38.33.72.97.72 1.96v2.9c0 .28.19.61.72.5A10.5 10.5 0 0 0 12 1.5z" }
            }
        },
        Host::Codeberg => rsx! {
            svg { width: "14", height: "14", view_box: "0 0 24 24", fill: "currentColor",
                path { d: "M12 2 3 19.5h3.2L12 8.7l5.8 10.8H21L12 2z" }
                path { d: "M12 11.6 8.2 19.5h7.6L12 11.6z", opacity: "0.55" }
            }
        },
    }
}
