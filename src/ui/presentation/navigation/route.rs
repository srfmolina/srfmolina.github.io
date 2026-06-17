//! The application's routes — the Dioxus mirror of Krocy's `route.kt`
//! (`KrocyRoute`) together with the route→screen binding from `AppNavGraph.kt`.
//!
//! Compose distributes that binding across per-feature `NavGraphBuilder`
//! extensions; Dioxus centralizes it here via `#[derive(Routable)]`. Each
//! variant renders the component named identically to it.

use dioxus::prelude::*;

use crate::ui::presentation::feature::home::home_screen::HomeScreen;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeScreen,
}
