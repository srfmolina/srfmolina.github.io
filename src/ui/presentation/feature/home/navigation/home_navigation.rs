//! The Dioxus analog of Krocy's `navigateToStock` helper.
//!
//! The actual route→screen binding lives in the central [`Route`] enum (Dioxus
//! centralizes routing), so this module only exposes the typed navigation target
//! that callers use with `Link { to: home_route() }` or `navigator().push(...)`.

use crate::ui::presentation::navigation::route::Route;

/// The route that navigates to the Home screen.
pub fn home_route() -> Route {
    Route::HomeScreen
}
