//! Shared chrome shown on every screen (header, footer). Rendered by the
//! router's `AppChrome` layout (Krocy's persistent `Scaffold`), not by
//! individual screens.

mod footer;
mod header;

pub use footer::Footer;
pub use header::Header;
