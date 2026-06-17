//! Minimal example: a counter screen built on the Krocy-style `BaseViewModel`.
//!
//! Run it with a renderer feature enabled, for example:
//!
//! ```bash
//! cargo run --example counter --features desktop
//! ```
//!
//! It shows the full round trip:
//! State (`CounterState`) + Event (`CounterEvent`) + Effect (`CounterEffect`)
//! -> a `CounterViewModel` implementing `BaseViewModel`
//! -> a `CounterScreen` that dispatches `launch_event` and consumes effects,
//!    plus a child component that reads the same state straight from context.

use dioxus::prelude::*;
use krocy_dioxus::base::*;
use krocy_dioxus::view_model;

// --- STATE (data class CounterState(...) : UiState) ---
#[derive(Clone, PartialEq)]
struct CounterState {
    count: i32,
}
impl UiState for CounterState {}

// --- EVENT (sealed interface CounterEvent : UiEvent) ---
enum CounterEvent {
    Increment,
    Decrement,
    Reset,
}
impl UiEvent for CounterEvent {}

// --- EFFECT (sealed interface CounterEffect : UiEffect) ---
enum CounterEffect {
    ShowMessage(String),
}
impl UiEffect for CounterEffect {}

// --- VIEWMODEL (class CounterViewModel : BaseViewModel<Event, State, Effect>()) ---
// The macro writes the struct + `from_core` / `core` (the ViewModelCarrier impl).
view_model!(CounterViewModel<CounterState, CounterEffect>);

// You only write the half that changes: Event, create_initial_state, handle_event.
impl BaseViewModel for CounterViewModel {
    type Event = CounterEvent;

    fn create_initial_state() -> CounterState {
        CounterState { count: 0 }
    }

    // override suspend fun handleEvent(event: CounterEvent)
    async fn handle_event(self, event: CounterEvent) {
        // In a real screen you might `.await` a repository here before reducing.
        match event {
            CounterEvent::Increment => {
                self.set_state(|s| CounterState { count: s.count + 1 });
            }
            CounterEvent::Decrement => {
                self.set_state(|s| CounterState { count: s.count - 1 });
            }
            CounterEvent::Reset => {
                self.set_state(|_| CounterState { count: 0 });
                self.launch_effect(CounterEffect::ShowMessage("Counter reset".into()));
            }
        }
    }
}

// --- SCREEN ---
#[component]
fn CounterScreen() -> Element {
    // Create + inject the ViewModel for this screen.
    let vm = use_view_model::<CounterViewModel>();

    // Consume one-shot effects (toast / navigation / logging).
    use_effects(vm, |effect| match effect {
        CounterEffect::ShowMessage(msg) => println!("[effect] {msg}"),
    });

    // Reactive read: `.read()` subscribes this component to state changes.
    let count = vm.state().read().count;

    rsx! {
        div {
            h1 { "Count: {count}" }
            button { onclick: move |_| vm.launch_event(CounterEvent::Decrement), "-" }
            button { onclick: move |_| vm.launch_event(CounterEvent::Reset), "Reset" }
            button { onclick: move |_| vm.launch_event(CounterEvent::Increment), "+" }
        }
        // A child that pulls the same ViewModel from context — no props needed.
        CounterLabel {}
    }
}

#[component]
fn CounterLabel() -> Element {
    let vm = use_context::<CounterViewModel>();
    let count = vm.state().read().count;
    rsx! { p { "(child component) the count is {count}" } }
}

fn main() {
    dioxus::launch(CounterScreen);
}
