//! Persistent header chrome — brand, in-page nav, theme toggle, language
//! switcher. Lives above the router in `App`, which owns the app ViewModel and
//! supplies the mutation callbacks as `EventHandler` props. The header never
//! sees a ViewModel.

use dioxus::prelude::*;

use crate::ui::app_view_model::Theme;
use crate::ui::i18n::{Language, Texts};

/// The site header. Reads localized labels from `Texts`; raises theme/language
/// changes through the `on_*` callbacks supplied by the parent.
#[component]
pub fn Header(
    language: Language,
    theme: Theme,
    on_toggle_theme: EventHandler<()>,
    on_set_language: EventHandler<Language>,
) -> Element {
    let texts = Texts::for_language(language);

    rsx! {
        header { class: "header",
            a { class: "brand", href: "#top",
                span { class: "brand-badge", "S" }
                span { class: "brand-name", "srfmolina" }
            }
            nav { class: "nav",
                a { class: "nav-link", href: "#projects", "{texts.nav.projects}" }
                a { class: "nav-link", href: "#stack", "{texts.nav.stack}" }
                button {
                    class: "theme-toggle",
                    "aria-label": "Toggle colour theme",
                    onclick: move |_| on_toggle_theme.call(()),
                    span { class: "theme-toggle-icon", ThemeIcon { dark: theme.is_dark() } }
                    if theme.is_dark() { "{texts.theme.light}" } else { "{texts.theme.dark}" }
                }
                LanguageDropdown { current: language, on_select: on_set_language }
            }
        }
    }
}

/// The header language switcher. Selecting a language calls `on_select`; it has
/// no knowledge of the ViewModel that ultimately handles the change.
#[component]
fn LanguageDropdown(current: Language, on_select: EventHandler<Language>) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "lang-switch",
            button {
                class: "lang-trigger",
                "aria-haspopup": "listbox",
                "aria-expanded": "{open()}",
                onclick: move |_| open.set(!open()),
                span { class: "lang-code", "{current.code()}" }
                span { class: "lang-caret", "▾" }
            }
            if open() {
                div { class: "lang-menu", role: "listbox",
                    for lang in Language::all().iter().copied() {
                        button {
                            key: "{lang.code()}",
                            class: if lang == current { "lang-option lang-option--active" } else { "lang-option" },
                            role: "option",
                            "aria-selected": "{lang == current}",
                            onclick: move |_| {
                                on_select.call(lang);
                                open.set(false);
                            },
                            "{lang.label()}"
                        }
                    }
                }
            }
        }
    }
}

/// The toggle's glyph: a sun for dark mode (press to go light), a moon for light.
#[component]
fn ThemeIcon(dark: bool) -> Element {
    if dark {
        rsx! {
            svg {
                width: "17", height: "17", view_box: "0 0 24 24",
                fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
                circle { cx: "12", cy: "12", r: "4" }
                path { d: "M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4" }
            }
        }
    } else {
        rsx! {
            svg {
                width: "16", height: "16", view_box: "0 0 24 24", fill: "currentColor",
                path { d: "M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8z" }
            }
        }
    }
}
