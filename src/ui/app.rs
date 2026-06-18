//! The application root — the Dioxus mirror of Krocy's `ui/App.kt`.
//!
//! Owns the persistent chrome (header + footer) shown on every screen and the
//! single source of app-global state (the `AppViewModel`). Routed screens never
//! receive the ViewModel; instead `App` provides the language and theme as
//! read-only context (`ReadSignal<Language>` / `ReadSignal<Theme>`), and feeds
//! the chrome its mutation callbacks as `EventHandler` props.

use dioxus::prelude::*;

use crate::ui::app_view_model::{AppEvent, AppViewModel, Theme};
use crate::ui::base::*;
use crate::ui::i18n::Language;
use crate::ui::presentation::common::{Footer, Header};
use crate::ui::presentation::navigation::NavigationComponent;

/// Bundled stylesheet (CSS custom properties, themes, component styles).
static MAIN_CSS: Asset = asset!("/assets/main.css");

/// The portfolio application root.
#[component]
pub fn App() -> Element {
    // App-global state. Created once; hydrate persisted language + theme.
    let app_vm = use_view_model::<AppViewModel>();
    use_hook(|| app_vm.launch_event(AppEvent::Init));

    // Expose language + theme to the routed subtree as read-only context.
    let app_state = app_vm.state();
    let language = use_memo(move || app_state.read().language);
    let theme = use_memo(move || app_state.read().theme);
    use_context_provider(|| ReadSignal::<Language>::from(language));
    use_context_provider(|| ReadSignal::<Theme>::from(theme));

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Spectral:ital,wght@0,400;0,500;0,600;1,400&family=Hanken+Grotesk:wght@400;500;600;700&family=Space+Mono:wght@400;700&display=swap"
        }

        div { class: "page", "data-theme": "{theme().data_attr()}",
            div { class: "grain", "aria-hidden": "true" }
            Header {
                language: language(),
                theme: theme(),
                on_toggle_theme: move |_| app_vm.launch_event(AppEvent::ToggleTheme),
                on_set_language: move |l| app_vm.launch_event(AppEvent::SetLanguage(l)),
            }
            NavigationComponent {}
            Footer { language: language() }
        }
    }
}
