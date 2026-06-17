//! Web portfolio built with Dioxus.
//!
//! The module tree mirrors Krocy's `com.srfmolina.krocy.ui` Kotlin module:
//! `ui::base`, `ui::presentation::feature`, `ui::presentation::navigation`, `ui::app`.

pub mod ui;

// The `view_model!` macro (in `ui::base::macros`) expands to `$crate::base::…`.
// Re-export the base module at the crate root so that path keeps resolving while
// the files themselves live, unchanged, under `ui::base`.
pub use ui::base;
