//! A contact link (from the JSON `extra` map), e.g. linkedin.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    pub name: String,
    pub url: String,
}
