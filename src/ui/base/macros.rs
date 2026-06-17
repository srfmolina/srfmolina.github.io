//! The [`view_model!`] macro, which writes the mechanical
//! [`ViewModelCarrier`](crate::base::ViewModelCarrier) boilerplate for you.

/// Generates a ViewModel struct plus its
/// [`ViewModelCarrier`](crate::base::ViewModelCarrier) implementation.
///
/// This is the part of a ViewModel that never changes — the `{ core }` struct and
/// the `from_core` / `core` accessors. After calling it, you only write the
/// interesting half by hand: `impl BaseViewModel for YourViewModel { ... }`.
///
/// # Example
///
/// ```rust,ignore
/// use krocy_dioxus::base::*;
/// use krocy_dioxus::view_model;
///
/// view_model!(pub CounterViewModel<CounterState, CounterEffect>);
///
/// impl BaseViewModel for CounterViewModel {
///     type Event = CounterEvent;
///     fn create_initial_state() -> CounterState { CounterState { count: 0 } }
///     async fn handle_event(self, event: CounterEvent) { /* match event { ... } */ }
/// }
/// ```
///
/// The macro expands to roughly:
///
/// ```rust,ignore
/// #[derive(Clone, Copy)]
/// pub struct CounterViewModel { core: ViewModelCore<CounterState, CounterEffect> }
///
/// impl ViewModelCarrier for CounterViewModel {
///     type State = CounterState;
///     type Effect = CounterEffect;
///     fn from_core(core: ViewModelCore<CounterState, CounterEffect>) -> Self { Self { core } }
///     fn core(self) -> ViewModelCore<CounterState, CounterEffect> { self.core }
/// }
/// ```
///
/// `$vis` (e.g. `pub`) and outer attributes / doc comments are forwarded to the
/// generated struct.
#[macro_export]
macro_rules! view_model {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident < $state:ty, $effect:ty $(,)? >
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy)]
        $vis struct $name {
            core: $crate::base::ViewModelCore<$state, $effect>,
        }

        impl $crate::base::ViewModelCarrier for $name {
            type State = $state;
            type Effect = $effect;

            fn from_core(
                core: $crate::base::ViewModelCore<$state, $effect>,
            ) -> Self {
                Self { core }
            }

            fn core(self) -> $crate::base::ViewModelCore<$state, $effect> {
                self.core
            }
        }
    };
}
