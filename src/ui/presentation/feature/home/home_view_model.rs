//! Home screen ViewModel — the Dioxus mirror of Krocy's `XViewModel.kt`,
//! built with the macro pattern from `reference/login.rs`.

use crate::ui::base::*;
use crate::view_model;

// --- STATE (data class HomeState(...) : UiState) ---
#[derive(Clone, PartialEq)]
pub struct HomeState {
    pub greeting: String,
}
impl UiState for HomeState {}

// --- EVENT (sealed interface HomeEvent : UiEvent) ---
pub enum HomeEvent {
    Init,
}
impl UiEvent for HomeEvent {}

// --- EFFECT (sealed interface HomeEffect : UiEffect) ---
// The hello-world screen emits no one-shot effects yet.
pub enum HomeEffect {}
impl UiEffect for HomeEffect {}

// --- VIEWMODEL ---
// One line writes the struct + `from_core` / `core` (the ViewModelCarrier impl).
view_model!(pub HomeViewModel<HomeState, HomeEffect>);

impl BaseViewModel for HomeViewModel {
    type Event = HomeEvent;

    fn create_initial_state() -> HomeState {
        HomeState {
            greeting: "Hello, world!".to_string(),
        }
    }

    // override suspend fun handleEvent(event: HomeEvent)
    async fn handle_event(self, event: HomeEvent) {
        match event {
            HomeEvent::Init => {
                self.set_state(|_| Self::create_initial_state());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_greets_the_world() {
        assert_eq!(
            HomeViewModel::create_initial_state().greeting,
            "Hello, world!"
        );
    }
}
