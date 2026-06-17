//! Web entry point. `dioxus::launch` selects the renderer enabled by the
//! active cargo feature (`web`).

use portfolio::ui::app::App;

fn main() {
    dioxus::launch(App);
}
