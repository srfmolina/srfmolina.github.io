//! The router host — the Dioxus mirror of Krocy's `NavigationComponent.kt`
//! (which wraps Compose's `NavHost`).

use dioxus::prelude::*;

use super::route::Route;

/// Renders the current route. The Dioxus equivalent of `NavHost(navController, ...)`.
#[component]
pub fn NavigationComponent() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
