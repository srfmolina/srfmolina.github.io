//! Home screen — the Dioxus mirror of Krocy's `XScreen.kt`.

use dioxus::prelude::*;

use super::home_view_model::{HomeEvent, HomeViewModel};
use crate::ui::base::*;

/// The portfolio landing page. Creates and injects the screen's ViewModel,
/// dispatches `Init` once on first render, and renders the greeting from state.
#[component]
pub fn HomeScreen() -> Element {
    // Create + inject the ViewModel for this screen (Compose's `koinViewModel()`).
    let vm = use_view_model::<HomeViewModel>();

    // LaunchedEffect(Unit) { viewModel.launchEvent(Event.Init) } — run exactly once.
    use_hook(|| vm.launch_event(HomeEvent::Init));

    // Reactive read: `.read()` subscribes this component to state changes.
    let greeting = vm.state().read().greeting.clone();

    rsx! {
        main {
            h1 { "{greeting}" }
        }
    }
}
