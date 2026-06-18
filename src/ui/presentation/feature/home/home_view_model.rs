//! Home screen ViewModel — the Dioxus mirror of Krocy's `XViewModel.kt`.
//!
//! The home content is now static, language-keyed data on the i18n `Texts`
//! tree, so this ViewModel currently holds no state and only keeps the standard
//! `Init` lifecycle event. It is retained as the per-feature template and as the
//! home for imminent screen-local state.

use crate::ui::base::*;
use crate::view_model;

// --- STATE ---
#[derive(Clone, PartialEq)]
pub struct HomeState {}
impl UiState for HomeState {}

// --- EVENT ---
pub enum HomeEvent {
    /// First render. Currently a no-op.
    Init,
}
impl UiEvent for HomeEvent {}

// --- EFFECT ---
pub enum HomeEffect {}
impl UiEffect for HomeEffect {}

// --- VIEWMODEL ---
view_model!(pub HomeViewModel<HomeState, HomeEffect>);

impl BaseViewModel for HomeViewModel {
    type Event = HomeEvent;

    fn create_initial_state() -> HomeState {
        HomeState {}
    }

    async fn handle_event(self, event: HomeEvent) {
        match event {
            HomeEvent::Init => {}
        }
    }
}
