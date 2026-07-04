//! The router host and the persistent `Scaffold` — the Dioxus mirror of Krocy's
//! `NavigationComponent.kt` (`NavHost`) plus `App.kt`'s `Scaffold`.
//!
//! `NavigationComponent` is the `NavHost` (`Router::<Route>`). `AppChrome` is the
//! `Scaffold`: it renders the persistent `Header`/`Footer` around the swapped
//! screen body (`Outlet`). Because route `Link`s must live inside the router,
//! the chrome lives here (inside the router) rather than in `App`.

use dioxus::prelude::*;

use super::route::Route;
use crate::ui::app_view_model::Theme;
use crate::ui::i18n::Language;
use crate::ui::presentation::common::{Footer, Header};

/// The header's mutation callbacks, provided by `App` via context so the chrome
/// (now inside the router) can still drive the app ViewModel. Only callbacks and
/// read-only signals cross the context boundary — never the ViewModel itself.
#[derive(Clone, Copy)]
pub struct ChromeCallbacks {
    pub on_toggle_theme: EventHandler<()>,
    pub on_set_language: EventHandler<Language>,
}

/// Renders the current route. The Dioxus equivalent of `NavHost(navController, ...)`.
#[component]
pub fn NavigationComponent() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

/// The persistent `Scaffold`: chrome around the swapped screen body (`Outlet`).
/// Reads language/theme and the header callbacks from context (all provided by
/// `App`), so it takes no props.
#[component]
pub fn AppChrome() -> Element {
    let language = use_context::<ReadSignal<Language>>();
    let theme = use_context::<ReadSignal<Theme>>();
    let callbacks = use_context::<ChromeCallbacks>();

    rsx! {
        Header {
            language: *language.read(),
            theme: *theme.read(),
            on_toggle_theme: callbacks.on_toggle_theme,
            on_set_language: callbacks.on_set_language,
        }
        Outlet::<Route> {}
        Footer { language: *language.read() }
    }
}
