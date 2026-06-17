//! The application root — the Dioxus mirror of Krocy's `ui/App.kt`.

use dioxus::prelude::*;

use crate::ui::presentation::navigation::NavigationComponent;

/// The portfolio application root. Krocy wraps this in `KrocyTheme { … }`;
/// here we render the navigation host directly.
#[component]
pub fn App() -> Element {
    rsx! {
        NavigationComponent {}
    }
}
