//! Equivalent to Krocy's `UiEffect.kt`:
//!
//! ```kotlin
//! package com.srfmolina.krocy.ui.base
//!
//! interface UiEffect
//! ```

/// Marker trait for one-shot side effects (navigation, toasts, ...), mirroring
/// Krocy's `interface UiEffect`.
///
/// Effects are emitted with `launch_effect` and consumed exactly once by the
/// screen through [`crate::base::use_effects`]. They only need to be `'static`.
///
/// ```rust,ignore
/// enum CounterEffect { ShowMessage(String) }
/// impl UiEffect for CounterEffect {}
/// ```
pub trait UiEffect: 'static {}
