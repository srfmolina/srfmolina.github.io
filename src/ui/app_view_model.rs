//! The application-level ViewModel — owns cross-cutting UI state (the active
//! `Language` and colour `Theme`). It is owned by `App`, which exposes the
//! language and theme to the routed subtree as read-only context
//! (`ReadSignal<Language>` / `ReadSignal<Theme>`) — the ViewModel itself is not
//! provided. The Dioxus mirror of an app-scoped Krocy ViewModel, built with the
//! same `view_model!` / `BaseViewModel` pattern.

use dioxus::prelude::*;

use crate::ui::base::*;
use crate::ui::i18n::Language;
use crate::view_model;

/// `localStorage` keys used to remember the user's choices across reloads.
const THEME_STORAGE_KEY: &str = "sf-theme";
const LANGUAGE_STORAGE_KEY: &str = "sf-language";

// --- THEME ---------------------------------------------------------------

/// The colour theme. `Dark` is the default until the user picks one.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    /// The value written to the page's `data-theme` attribute.
    pub fn data_attr(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    /// The opposite theme — what a single toggle press selects.
    pub fn toggled(self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Theme::Dark)
    }

    /// Parse a persisted `localStorage` value back into a theme.
    fn from_stored(value: &str) -> Option<Theme> {
        match value {
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            _ => None,
        }
    }
}

// --- STATE / EVENT / EFFECT ---------------------------------------------

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub language: Language,
    pub theme: Theme,
}
impl UiState for AppState {}

pub enum AppEvent {
    /// First render: hydrate language and theme from `localStorage`.
    Init,
    /// User pressed the theme toggle.
    ToggleTheme,
    /// User picked a language from the switcher.
    SetLanguage(Language),
}
impl UiEvent for AppEvent {}

pub enum AppEffect {}
impl UiEffect for AppEffect {}

// --- VIEWMODEL -----------------------------------------------------------

view_model!(pub AppViewModel<AppState, AppEffect>);

impl BaseViewModel for AppViewModel {
    type Event = AppEvent;

    fn create_initial_state() -> AppState {
        AppState { language: Language::default(), theme: Theme::Dark }
    }

    async fn handle_event(self, event: AppEvent) {
        match event {
            AppEvent::Init => {
                if let Some(theme) = load_persisted_theme().await {
                    self.set_state(|s| AppState { theme, ..s.clone() });
                }
                if let Some(language) = load_persisted_language().await {
                    self.set_state(|s| AppState { language, ..s.clone() });
                }
            }
            AppEvent::ToggleTheme => {
                self.set_state(|s| AppState { theme: s.theme.toggled(), ..s.clone() });
                persist_theme(self.current_state().theme);
            }
            AppEvent::SetLanguage(language) => {
                self.set_state(|s| AppState { language, ..s.clone() });
                persist_language(language);
            }
        }
    }
}

// --- PERSISTENCE (no-ops outside the web renderer) -----------------------

async fn load_persisted_theme() -> Option<Theme> {
    let mut eval = document::eval(&format!(
        "dioxus.send(localStorage.getItem('{THEME_STORAGE_KEY}'));"
    ));
    match eval.recv::<Option<String>>().await {
        Ok(Some(stored)) => Theme::from_stored(&stored),
        _ => None,
    }
}

fn persist_theme(theme: Theme) {
    let _ = document::eval(&format!(
        "try {{ localStorage.setItem('{}', '{}'); }} catch (e) {{}}",
        THEME_STORAGE_KEY,
        theme.data_attr()
    ));
}

async fn load_persisted_language() -> Option<Language> {
    let mut eval = document::eval(&format!(
        "dioxus.send(localStorage.getItem('{LANGUAGE_STORAGE_KEY}'));"
    ));
    match eval.recv::<Option<String>>().await {
        Ok(Some(stored)) => Language::from_code(&stored),
        _ => None,
    }
}

fn persist_language(language: Language) {
    let _ = document::eval(&format!(
        "try {{ localStorage.setItem('{}', '{}'); }} catch (e) {{}}",
        LANGUAGE_STORAGE_KEY,
        language.code()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_english_and_dark() {
        let s = AppViewModel::create_initial_state();
        assert_eq!(s.language, Language::English);
        assert_eq!(s.theme, Theme::Dark);
    }

    #[test]
    fn toggling_flips_between_light_and_dark() {
        assert_eq!(Theme::Light.toggled(), Theme::Dark);
        assert_eq!(Theme::Dark.toggled(), Theme::Light);
    }

    #[test]
    fn data_attr_matches_the_css_selector_values() {
        assert_eq!(Theme::Light.data_attr(), "light");
        assert_eq!(Theme::Dark.data_attr(), "dark");
    }
}
