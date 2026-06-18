//! Dioxus 0.7 port of Krocy's `BaseViewModel.kt`.
//!
//! The original (abridged):
//!
//! ```kotlin
//! abstract class BaseViewModel<Event : UiEvent, State : UiState, Effect : UiEffect> : ViewModel() {
//!     abstract fun createInitialState(): State
//!
//!     private val _state by lazy { MutableStateFlow(createInitialState()) }
//!     val state: StateFlow<State> get() = _state.asStateFlow()
//!     val currentState: State get() = _state.value
//!
//!     private val _effect = Channel<Effect>(Channel.BUFFERED)
//!     val effect = _effect.receiveAsFlow()
//!
//!     fun launchEvent(event: Event) { viewModelScope.launch { handleEvent(event) } }
//!     abstract suspend fun handleEvent(event: Event)
//!
//!     protected fun setState(reduce: State.() -> State) { _state.update { it.reduce() } }
//!     protected fun launchEffect(effect: Effect) { viewModelScope.launch { _effect.send(effect) } }
//! }
//! ```
//!
//! ## How Krocy maps to Dioxus
//!
//! | Krocy (KMP / Compose)                    | Dioxus 0.7 port                                            |
//! |------------------------------------------|------------------------------------------------------------|
//! | `ViewModel` lifecycle                    | the screen component's scope                               |
//! | `viewModelScope`                         | [`spawn`] (tasks are cancelled when the component unmounts)|
//! | `MutableStateFlow` / `StateFlow`         | `Signal<State>`, exposed read-only as `ReadSignal<State>`  |
//! | `Channel(BUFFERED)` + `receiveAsFlow()`  | `futures` unbounded mpsc (buffered, one-shot, single consumer) |
//! | `launchEvent` / `handleEvent`            | [`BaseViewModel::launch_event`] / [`BaseViewModel::handle_event`] |
//! | `setState { reduce }`                    | [`BaseViewModel::set_state`]                               |
//! | `launchEffect`                           | [`BaseViewModel::launch_effect`]                           |
//! | `currentState`                           | [`BaseViewModel::current_state`]                           |
//!
//! ## Two traits, one of them generated
//!
//! The boilerplate (`from_core` / `core` and the `{ core }` struct) is identical
//! for every screen, so it lives in its own mechanical supertrait
//! [`ViewModelCarrier`], which the [`view_model!`](crate::view_model) macro
//! implements for you. You only implement [`BaseViewModel`] — the part that
//! actually changes: the `Event` type, `create_initial_state` and `handle_event`.

use dioxus::prelude::*;
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::StreamExt;
use std::future::Future;

use super::{UiEffect, UiEvent, UiState};

/// The shared internals of a ViewModel: the state signal plus the two ends of the
/// effect channel.
///
/// It is `Copy` because every field is a `Copy` Dioxus handle (`Signal` /
/// `CopyValue`). That is what lets a whole ViewModel be moved freely into event
/// handlers, spawned futures and child components (through context) without any
/// `Rc`/`Arc` ceremony.
///
/// You never build this by hand; [`use_view_model`] does it and hands it to your
/// `from_core` constructor (which the [`view_model!`](crate::view_model) macro writes).
pub struct ViewModelCore<State: UiState, Effect: UiEffect> {
    /// Single source of truth. Equivalent to Krocy's `_state` (`MutableStateFlow`).
    state: Signal<State>,
    /// Sender end of the one-shot effect channel. Equivalent to Krocy's `_effect`.
    effect_tx: CopyValue<UnboundedSender<Effect>>,
    /// Receiver end, taken exactly once by [`use_effects`].
    /// Equivalent to Krocy's `effect = _effect.receiveAsFlow()`.
    effect_rx: Signal<Option<UnboundedReceiver<Effect>>>,
}

// `#[derive(Copy)]` would wrongly require `State: Copy, Effect: Copy`, so we
// implement it by hand: the *fields* are always `Copy`, regardless of inner types.
impl<State: UiState, Effect: UiEffect> Clone for ViewModelCore<State, Effect> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<State: UiState, Effect: UiEffect> Copy for ViewModelCore<State, Effect> {}

impl<State: UiState, Effect: UiEffect> ViewModelCore<State, Effect> {
    /// Push an effect into the channel. `unbounded_send` never blocks, mirroring
    /// the buffered, one-shot delivery of Krocy's `Channel(Channel.BUFFERED)`.
    fn send_effect(self, effect: Effect) {
        let sender = self.effect_tx.read().clone();
        let _ = sender.unbounded_send(effect);
    }
}

/// The mechanical half of a ViewModel: it just carries a [`ViewModelCore`].
///
/// This is pure boilerplate — `from_core` is always `Self { core }` and `core`
/// is always `self.core` — so you normally generate it with the
/// [`view_model!`](crate::view_model) macro instead of writing it. It is split
/// out from [`BaseViewModel`] precisely so the macro can implement it on its own.
pub trait ViewModelCarrier: Copy + 'static {
    /// Screen state. Mirrors the `State : UiState` type parameter.
    type State: UiState;
    /// One-shot side effects. Mirrors the `Effect : UiEffect` type parameter.
    type Effect: UiEffect;

    /// Build the carrier from its core. Always `Self { core }`.
    fn from_core(core: ViewModelCore<Self::State, Self::Effect>) -> Self;

    /// Read the core back out. Always `self.core`.
    fn core(self) -> ViewModelCore<Self::State, Self::Effect>;
}

/// The Dioxus equivalent of Krocy's
/// `abstract class BaseViewModel<Event, State, Effect>` — the half you actually write.
///
/// `State` and `Effect` come from the [`ViewModelCarrier`] supertrait (generated by
/// the macro). Here you provide only what differs per screen: the `Event` type,
/// `create_initial_state` and `handle_event`. The `protected` helpers
/// (`launch_event`, `set_state`, `launch_effect`, `current_state`, `state`) are free.
///
/// ```rust,ignore
/// view_model!(CounterViewModel<CounterState, CounterEffect>); // generates the carrier
///
/// impl BaseViewModel for CounterViewModel {
///     type Event = CounterEvent;
///     fn create_initial_state() -> CounterState { CounterState { count: 0 } }
///     async fn handle_event(self, event: CounterEvent) { /* match ... */ }
/// }
/// ```
pub trait BaseViewModel: ViewModelCarrier {
    /// User intents. Mirrors the `Event : UiEvent` type parameter.
    type Event: UiEvent;

    /// `abstract fun createInitialState(): State`.
    fn create_initial_state() -> Self::State;

    /// `abstract suspend fun handleEvent(event: Event)`.
    ///
    /// Implement it with an `async fn`. Inside, call [`set_state`](Self::set_state)
    /// to reduce state and [`launch_effect`](Self::launch_effect) to emit effects.
    /// You can `.await` repositories / network calls freely — the whole body runs
    /// in its own task.
    fn handle_event(self, event: Self::Event) -> impl Future<Output = ()> + 'static;

    // ----------------------------------------------------------------------
    // Provided helpers — the Dioxus mirror of the Kotlin base class members.
    // ----------------------------------------------------------------------

    /// `fun launchEvent(event)` — the single entry point from the UI.
    ///
    /// Each call spawns its **own** task (`viewModelScope.launch { handleEvent(event) }`),
    /// so multiple events can be in flight at once. The task is automatically
    /// cancelled if the owning component unmounts (Dioxus' answer to `onCleared`).
    fn launch_event(self, event: Self::Event) {
        spawn(async move {
            self.handle_event(event).await;
        });
    }

    /// `protected fun setState(reduce: State.() -> State)`.
    ///
    /// Reads the current value **without subscribing** (`peek`), applies `reduce`,
    /// then writes the new value, notifying every component that reads the state.
    fn set_state(self, reduce: impl FnOnce(&Self::State) -> Self::State) {
        let mut state = self.core().state;
        let next = {
            let current = state.peek();
            reduce(&*current)
        };
        state.set(next);
    }

    /// `protected fun launchEffect(effect)`.
    ///
    /// Emits a one-shot effect in its own task, matching Krocy's
    /// `viewModelScope.launch { _effect.send(effect) }`.
    fn launch_effect(self, effect: Self::Effect) {
        let core = self.core();
        spawn(async move {
            core.send_effect(effect);
        });
    }

    /// `val currentState: State` — a non-reactive snapshot of the current state.
    /// Uses `peek`, so reading it does **not** subscribe the caller.
    fn current_state(self) -> Self::State {
        let state = self.core().state;
        let cloned = (*state.peek()).clone();
        cloned
    }

    /// `val state: StateFlow<State>` — a read-only, reactive view of the state.
    ///
    /// Read it inside `rsx!` (e.g. `vm.state().read().count`) to subscribe the
    /// component to updates, or pass it to a child typed as `ReadSignal<State>`.
    fn state(self) -> ReadSignal<Self::State> {
        self.core().state.into()
    }
}

/// Creates the ViewModel for the current screen.
///
/// The ViewModel is **not** provided to the subtree: descendants never fetch a
/// ViewModel from context. To let a child trigger one of this ViewModel's
/// events, pass the launching function down as an `EventHandler` prop. The one
/// app-global exception (read-only `Language` / `Theme`) is provided explicitly
/// by `App`, not here.
///
/// ```rust,ignore
/// #[component]
/// fn CounterScreen() -> Element {
///     let vm = use_view_model::<CounterViewModel>();
///     // ...
/// }
/// ```
pub fn use_view_model<VM: BaseViewModel>() -> VM {
    // MutableStateFlow(createInitialState()) -> a Signal initialised once.
    let state = use_signal(VM::create_initial_state);

    // Channel(Channel.BUFFERED) -> an unbounded mpsc, created exactly once and
    // stored in Copy handles so the core stays Copy.
    let (effect_tx, effect_rx) = use_hook(|| {
        let (tx, rx) = unbounded::<VM::Effect>();
        (CopyValue::new(tx), Signal::new(Some(rx)))
    });

    let vm = VM::from_core(ViewModelCore {
        state,
        effect_tx,
        effect_rx,
    });

    vm
}

/// Collects one-shot effects for the lifetime of the current component — the
/// Dioxus mirror of `LaunchedEffect(Unit) { vm.effect.collect { effect -> ... } }`.
///
/// The receiver is taken exactly **once**, so call this in the screen that owns
/// the ViewModel (the one that called [`use_view_model`]).
///
/// ```rust,ignore
/// use_effects(vm, |effect| match effect {
///     CounterEffect::ShowMessage(msg) => { /* show a toast, navigate, ... */ }
/// });
/// ```
pub fn use_effects<VM, H>(vm: VM, handler: H)
where
    VM: BaseViewModel,
    H: FnMut(VM::Effect) + 'static,
{
    use_hook(move || {
        spawn(async move {
            let mut handler = handler;
            let mut rx_holder = vm.core().effect_rx;
            // Take the single receiver (one-shot, like receiveAsFlow()).
            let receiver = rx_holder.write().take();
            if let Some(mut rx) = receiver {
                while let Some(effect) = rx.next().await {
                    handler(effect);
                }
            }
        });
    });
}
