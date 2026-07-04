//! A single project/link shown in the portfolio.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectLink {
    pub url: String,
    pub description: String,
    pub language: Option<String>,
}
