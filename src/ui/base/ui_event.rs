//! Equivalent to Krocy's `UiEvent.kt`:
//!
//! ```kotlin
//! package com.srfmolina.krocy.ui.base
//!
//! interface UiEvent
//! ```

/// Marker trait for UI events (user intents), mirroring Krocy's `interface UiEvent`.
///
/// Events are dispatched through `launch_event`, each in its own task. They only
/// need to be `'static` so they can be moved into the spawned future.
///
/// ```rust,ignore
/// enum CounterEvent { Increment, Decrement, Reset }
/// impl UiEvent for CounterEvent {}
/// ```
pub trait UiEvent: 'static {}
