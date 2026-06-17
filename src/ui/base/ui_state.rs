//! Equivalent to Krocy's `UiState.kt`:
//!
//! ```kotlin
//! package com.srfmolina.krocy.ui.base
//!
//! interface UiState
//! ```

/// Marker trait for a screen's state, mirroring Krocy's `interface UiState`.
///
/// Unlike the empty Kotlin interface, the Dioxus port asks for three small
/// bounds, all of which Dioxus needs to manage reactive state efficiently:
///
/// - `Clone`     — so we can hand out a snapshot via `current_state()`.
/// - `PartialEq` — so Dioxus can skip re-renders when the state is unchanged.
/// - `'static`   — so the state can live inside a `Signal`.
///
/// Implement it on a plain data struct, exactly like `data class FooState(...) : UiState`:
///
/// ```rust,ignore
/// #[derive(Clone, PartialEq)]
/// struct CounterState { count: i32 }
/// impl UiState for CounterState {}
/// ```
pub trait UiState: Clone + PartialEq + 'static {}
