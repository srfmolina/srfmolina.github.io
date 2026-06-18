//! The `i18n` module — cross-cutting localization for the UI.
//!
//! Holds the [`Language`] enum and the per-feature typed string tables. There is
//! no i18n crate and no JSON: translations are `&'static str` resolved by a
//! `match` on [`Language`], so forgetting a string in one language is a *compile*
//! error, and all of it is pure and host-testable.

mod language;
mod texts;

pub use language::Language;
pub use texts::{Texts, Project, Tech, ENGLISH_TEXTS, SPANISH_TEXTS};
