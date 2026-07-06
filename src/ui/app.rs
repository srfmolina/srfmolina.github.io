//! The application root — the Dioxus mirror of Krocy's `ui/App.kt`.
//!
//! Owns the single source of app-global state (`AppViewModel`) and the themed
//! page wrapper. The persistent chrome (header/footer) is the router's `Scaffold`
//! (`AppChrome`), so `App` provides what the chrome needs as context — read-only
//! `Language`/`Theme` and the header callbacks (`ChromeCallbacks`) — plus the
//! app's use cases (`UseCases`), and renders the `NavHost`.
//! It also dismisses the static `index.html` splash once the stylesheet loads.

use dioxus::prelude::*;

use crate::ui::app_view_model::{AppEvent, AppViewModel, Theme};
use crate::ui::base::*;
use crate::ui::i18n::Language;
use crate::ui::presentation::navigation::navigation_component::ChromeCallbacks;
use crate::ui::presentation::navigation::NavigationComponent;

/// Bundled stylesheet (CSS custom properties, themes, component styles).
static MAIN_CSS: Asset = asset!("/assets/main.css");

/// Dismisses the static `#splash` overlay (see `index.html`) once the bundled
/// stylesheet is loaded. `{MAIN_CSS_URL}` is substituted with the hashed asset
/// URL before eval. Ready = already in `document.styleSheets` (cache hit —
/// removed instantly, no fade: the splash should not outlive the load it
/// covers), or the link's `load`/`error` event (fade), or a 4 s timeout — so
/// a broken CSS fetch can never strand the splash. No-op when the splash is
/// already gone.
const SPLASH_DISMISS_JS: &str = r#"
(function () {
  var splash = document.getElementById('splash');
  if (!splash) { return; }
  var done = false;
  function remove() {
    if (splash.parentNode) { splash.parentNode.removeChild(splash); }
  }
  function dismiss(instant) {
    if (done) { return; }
    done = true;
    if (instant) { remove(); return; }
    splash.classList.add('splash-out');
    splash.addEventListener('transitionend', remove);
    setTimeout(remove, 400);
  }
  var url = '{MAIN_CSS_URL}';
  for (var i = 0; i < document.styleSheets.length; i++) {
    var href = document.styleSheets[i].href;
    if (href && href.indexOf(url) !== -1) { dismiss(true); return; }
  }
  var link = document.querySelector('link[rel="stylesheet"][href="' + url + '"]');
  if (link) {
    link.addEventListener('load', function () { dismiss(false); });
    link.addEventListener('error', function () { dismiss(false); });
  }
  setTimeout(function () { dismiss(false); }, 4000);
})();
"#;

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

    // Header callbacks for the chrome (which now lives inside the router).
    let on_toggle_theme = EventHandler::new(move |_| app_vm.launch_event(AppEvent::ToggleTheme));
    let on_set_language = EventHandler::new(move |l: Language| app_vm.launch_event(AppEvent::SetLanguage(l)));
    use_context_provider(move || ChromeCallbacks { on_toggle_theme, on_set_language });
    use_context_provider(|| crate::data::UseCases::new());

    // Fade out the static splash (index.html) once the stylesheet is ready.
    // Runs once after first render — reads no signals, so it never re-runs.
    use_effect(|| {
        document::eval(&SPLASH_DISMISS_JS.replace("{MAIN_CSS_URL}", &MAIN_CSS.to_string()));
    });

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Spectral:ital,wght@0,400;0,500;0,600;1,400&family=Hanken+Grotesk:wght@400;500;600;700&family=Space+Mono:wght@400;700&display=swap"
        }

        div { class: "page", "data-theme": "{theme().data_attr()}",
            div { class: "grain", "aria-hidden": "true" }
            NavigationComponent {}
        }
    }
}
