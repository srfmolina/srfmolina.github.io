//! Shared chrome shown on every screen (header, footer). Rendered by `App`
//! around the router, not by individual screens.

mod footer;
mod header;

pub use footer::Footer;
pub use header::Header;
