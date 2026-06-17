# krocy-dioxus

A small, reusable port of [Krocy](https://github.com/srfmolina/Krocy)'s
`BaseViewModel` pattern to **Dioxus 0.7** (Rust). It keeps Krocy's exact
vocabulary — `UiState` / `UiEvent` / `UiEffect`, a `BaseViewModel` with
`launch_event`, `launch_effect`, `set_state`, `current_state`, and a
`handle_event` you override per screen — so the mental model carries over 1:1
from the Kotlin Multiplatform original.

This folder is the Dioxus equivalent of Krocy's `com.srfmolina.krocy.ui.base`
package (`BaseViewModel.kt`, `UiState.kt`, `UiEvent.kt`, `UiEffect.kt`).

## How Krocy maps to Dioxus

| Krocy (KMP / Compose)                    | This crate (Dioxus 0.7)                                        |
|------------------------------------------|---------------------------------------------------------------|
| `ViewModel` lifecycle                    | the screen component's scope                                   |
| `viewModelScope`                         | `spawn` (tasks are cancelled when the component unmounts)      |
| `MutableStateFlow` / `StateFlow`         | `Signal<State>`, exposed read-only as `ReadSignal<State>`      |
| `Channel(BUFFERED)` + `receiveAsFlow()`  | `futures` unbounded mpsc (buffered, one-shot, single consumer) |
| `launchEvent` / `handleEvent`            | `launch_event` (one task per event) / `handle_event`          |
| `setState { reduce }`                    | `set_state(\|s\| ...)`                                          |
| `launchEffect`                           | `launch_effect`                                               |
| `currentState`                           | `current_state`                                               |
| `LaunchedEffect { vm.effect.collect {} }`| `use_effects(vm, \|effect\| ...)`                              |

Like Krocy, `launch_event` starts a **fresh task per event**
(`viewModelScope.launch { handleEvent(event) }`), so events run concurrently
rather than being serialised through a single channel.

## Using it (three steps per screen)

1. Declare your `State` / `Event` / `Effect` and mark them with the traits.
2. Generate the ViewModel struct + boilerplate with one line:
   `view_model!(YourViewModel<YourState, YourEffect>);`, then `impl BaseViewModel`
   for it, overriding only `create_initial_state` and `async fn handle_event`
   (and the `Event` associated type).
3. In the screen component: `let vm = use_view_model::<YourViewModel>();`,
   optionally `use_effects(vm, ...)`, read state with `vm.state().read()`, and
   dispatch with `vm.launch_event(...)`.

The `view_model!` macro writes the repetitive `from_core` / `core` accessors (the
`ViewModelCarrier` impl), which are identical for every screen. If you prefer no
macro, you can implement `ViewModelCarrier` by hand instead — it's four lines.

See [`examples/counter.rs`](examples/counter.rs) for the full, minimal example.

## Running the example

The library is renderer-agnostic; the example launches a window, so pick a
renderer feature:

```bash
cargo run --example counter --features desktop
```

(or `--features web` and serve with `dx serve`).

## Important caveats

- **Not compiled in the environment it was generated in.** The code targets the
  Dioxus 0.7 APIs verified against the official docs (`Signal`, `ReadSignal`,
  `CopyValue`, `spawn`, `use_signal`, `use_hook`, `use_context_provider`,
  `peek`/`read`/`set`). Pin your exact version in `Cargo.toml` and run
  `cargo build` once; if a symbol name differs in your point release (Dioxus has
  renamed signal types across versions — e.g. the older `ReadOnlySignal`), adjust
  the import.
- **`spawn` binds to the *current* scope, not a dedicated `viewModelScope`.**
  Tasks are cancelled when the component that triggered them unmounts. For the
  usual case (events dispatched from the screen that owns the VM) this matches
  Krocy closely. If you dispatch from a short-lived child and need the task to
  outlive it, capture the owning `ScopeId` at `use_view_model` time and spawn
  into it.
- **Effects are one-shot and single-consumer**, exactly like `receiveAsFlow()`:
  call `use_effects` once, in the screen that owns the ViewModel.
