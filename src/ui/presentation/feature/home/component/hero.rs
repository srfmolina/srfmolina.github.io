use dioxus::prelude::*;

use crate::ui::i18n::{Language, Texts};

/// Portrait shown in the hero media slot.
static PROFILE: Asset = asset!("/assets/profile.jpeg");

#[component]
pub fn Hero() -> Element {

    let language = use_context::<ReadSignal<Language>>();
    let texts = Texts::for_language(*language.read());
    let home = &texts.home;

    rsx! {

        section { class: "hero",
            div { class: "hero-copy",
                div { class: "eyebrow",
                    span { class: "eyebrow-rule" }
                    "{home.hero.eyebrow}"
                }
                h1 { class: "hero-title", "Serafín", br {} "L. Molina" }
                p { class: "hero-text",
                    span { class: "accent-ink", "{home.hero.lead_accent}" }
                    "{home.hero.lead_rest}"
                }
                p { class: "hero-text hero-text--last",
                    "{home.hero.para2_prefix}"
                    span { class: "accent-ink", "NTT Data" }
                    "{home.hero.para2_suffix}"
                }
                div { class: "chips",
                    span { class: "chip",
                        span { class: "chip-dot" }
                        "{home.hero.chip_role}"
                    }
                    span { class: "chip", "{home.hero.chip_platforms}" }
                }
            }
            div { class: "hero-media",
                div { class: "portrait",
                    div { class: "portrait-frame-a", "aria-hidden": "true" }
                    div { class: "portrait-frame-b", "aria-hidden": "true" }
                    div { class: "portrait-slot",
                        img {
                            src: PROFILE,
                            alt: "Serafín López Molina",
                            loading: "lazy",
                        }
                    }
                }
            }
        }
    }

}
