//! Home screen — the Dioxus mirror of Krocy's `XScreen.kt`.
//!
//! Renders only the routed `<main>` content; the surrounding page chrome
//! (header/footer/theme attribute) is owned by `App`. Reads the active language
//! from the read-only context `App` provides, then resolves the localized
//! `Texts` tree. Keeps an inert `HomeViewModel` as the per-feature template.

use dioxus::prelude::*;

use super::home_view_model::{HomeEvent, HomeViewModel};
use crate::ui::base::*;
use crate::ui::i18n::{Language, Texts};
use crate::ui::presentation::feature::home::component::hero::Hero;

/// The portfolio landing page (main content only).
#[component]
pub fn HomeScreen() -> Element {
    let language = use_context::<ReadSignal<Language>>();

    let vm = use_view_model::<HomeViewModel>();
    use_hook(|| vm.launch_event(HomeEvent::Init));

    let texts = Texts::for_language(*language.read());
    let home = &texts.home;

    rsx! {
        main { class: "main", id: "top",
            
            Hero{}

            section { class: "section", id: "projects",
                div { class: "section-head",
                    h2 { class: "section-title", "{home.projects_title}" }
                    span { class: "section-note", "{home.projects_note}" }
                }
                div { class: "project-grid",
                    for p in home.projects.iter() {
                        article { key: "{p.name}", class: "project-card", tabindex: "0",
                            div { class: "project-card-top",
                                span { class: "project-no", "{p.no}" }
                                span { class: "project-lang", "{p.lang}" }
                            }
                            h3 { class: "project-name", "{p.name}" }
                            p { class: "project-desc", "{p.desc}" }
                            div { class: "project-tag",
                                "{p.tag}"
                                span { class: "project-tag-arrow", "→" }
                            }
                        }
                    }
                }
            }

            section { class: "section section--stack", id: "stack",
                div { class: "section-head",
                    h2 { class: "section-title", "{home.stack_title}" }
                    span { class: "section-note", "{home.stack_note}" }
                }
                div { class: "tech-grid",
                    for t in home.techs.iter() {
                        div { key: "{t.name}", class: "tech-card", tabindex: "0",
                            span { class: "tech-cat", "{t.cat}" }
                            span { class: "tech-name", "{t.name}" }
                        }
                    }
                }
            }
        }
    }
}
