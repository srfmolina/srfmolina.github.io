//! Persistent footer chrome. Reads its single localized string from `Texts`.

use dioxus::prelude::*;

use crate::ui::i18n::{Language, Texts};

/// The site footer.
#[component]
pub fn Footer(language: Language) -> Element {
    let texts = Texts::for_language(language);

    rsx! {
        footer { class: "footer",
            span { class: "footer-name", "Serafín L. Molina (@srfmolina)" }
            span { class: "footer-meta", "{texts.footer.meta}" }
        }
    }
}
