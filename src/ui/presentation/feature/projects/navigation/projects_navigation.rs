//! The Dioxus analog of Krocy's `navigateToX` helper for the Projects screen.
//!
//! The route→screen binding lives in the central [`Route`] enum; this exposes the
//! typed target used with `Link { to: projects_route() }` or `navigator().push(...)`.

use crate::ui::presentation::navigation::route::Route;

/// The route that navigates to the Projects screen.
pub fn projects_route() -> Route {
    Route::ProjectsScreen
}
