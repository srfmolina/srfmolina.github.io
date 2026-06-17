//! Second example: an async login form built on the Krocy-style `BaseViewModel`.
//!
//! Run it with a renderer feature enabled, for example:
//!
//! ```bash
//! cargo run --example login --features desktop
//! ```
//!
//! What this adds over `counter.rs`:
//! - a **truly async** `handle_event` that `.await`s a (fake) `authenticate` call,
//! - multi-field state (`username`, `password`, `loading`, `error`),
//! - reading `current_state()` inside the handler to grab the typed-in values,
//! - two effects, one for success and one for failure.
//!
//! Credentials that "work": username `admin`, password `secret`.

use dioxus::prelude::*;
use krocy_dioxus::base::*;
use krocy_dioxus::view_model;

// --- STATE ---
#[derive(Clone, PartialEq)]
struct LoginState {
    username: String,
    password: String,
    loading: bool,
    error: Option<String>,
}
impl UiState for LoginState {}

// --- EVENT ---
enum LoginEvent {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
}
impl UiEvent for LoginEvent {}

// --- EFFECT ---
enum LoginEffect {
    LoggedIn(String), // e.g. navigate to the home screen
    ShowError(String),
}
impl UiEffect for LoginEffect {}

// --- VIEWMODEL ---
// One line writes the struct + `from_core` / `core` (the ViewModelCarrier impl).
view_model!(LoginViewModel<LoginState, LoginEffect>);

impl BaseViewModel for LoginViewModel {
    type Event = LoginEvent;

    fn create_initial_state() -> LoginState {
        LoginState {
            username: String::new(),
            password: String::new(),
            loading: false,
            error: None,
        }
    }

    // override suspend fun handleEvent(event: LoginEvent)
    async fn handle_event(self, event: LoginEvent) {
        match event {
            LoginEvent::UsernameChanged(username) => {
                self.set_state(|s| LoginState {
                    username,
                    ..s.clone()
                });
            }
            LoginEvent::PasswordChanged(password) => {
                self.set_state(|s| LoginState {
                    password,
                    ..s.clone()
                });
            }
            LoginEvent::Submit => {
                // Flip into the loading state and clear any previous error.
                self.set_state(|s| LoginState {
                    loading: true,
                    error: None,
                    ..s.clone()
                });

                // Snapshot the current values, then await the "request".
                let snapshot = self.current_state();
                match authenticate(&snapshot.username, &snapshot.password).await {
                    Ok(user) => {
                        self.set_state(|s| LoginState {
                            loading: false,
                            ..s.clone()
                        });
                        self.launch_effect(LoginEffect::LoggedIn(user));
                    }
                    Err(message) => {
                        self.set_state(|s| LoginState {
                            loading: false,
                            error: Some(message.clone()),
                            ..s.clone()
                        });
                        self.launch_effect(LoginEffect::ShowError(message));
                    }
                }
            }
        }
    }
}

/// Stand-in for a real auth call. A real implementation would `.await` network
/// I/O here; this one resolves immediately so the example stays dependency-free.
async fn authenticate(username: &str, password: &str) -> Result<String, String> {
    if username == "admin" && password == "secret" {
        Ok(username.to_string())
    } else {
        Err("Invalid username or password".to_string())
    }
}

// --- SCREEN ---
#[component]
fn LoginScreen() -> Element {
    let vm = use_view_model::<LoginViewModel>();

    // One-shot effects: navigate on success, toast on error.
    use_effects(vm, |effect| match effect {
        LoginEffect::LoggedIn(user) => println!("[effect] logged in -> go home, welcome {user}"),
        LoginEffect::ShowError(msg) => println!("[effect] toast: {msg}"),
    });

    let state = vm.state();
    let s = state.read();
    let button_label = if s.loading { "Signing in..." } else { "Sign in" };

    rsx! {
        div {
            h1 { "Login" }
            input {
                placeholder: "username",
                value: "{s.username}",
                oninput: move |e| vm.launch_event(LoginEvent::UsernameChanged(e.value())),
            }
            input {
                r#type: "password",
                placeholder: "password",
                value: "{s.password}",
                oninput: move |e| vm.launch_event(LoginEvent::PasswordChanged(e.value())),
            }
            button {
                disabled: s.loading,
                onclick: move |_| vm.launch_event(LoginEvent::Submit),
                "{button_label}"
            }
            // Render the error only when present.
            {s.error.as_ref().map(|err| rsx! {
                p { style: "color: red;", "{err}" }
            })}
        }
    }
}

fn main() {
    dioxus::launch(LoginScreen);
}
