//! The link catalog aggregate (Krocy's `domain.model`).

use super::{contact::Contact, project_link::ProjectLink};

/// The full link catalog, grouped by host plus contacts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Links {
    pub github: Vec<ProjectLink>,
    pub codeberg: Vec<ProjectLink>,
    pub contacts: Vec<Contact>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_links_is_empty() {
        let links = Links::default();
        assert!(links.github.is_empty());
        assert!(links.codeberg.is_empty());
        assert!(links.contacts.is_empty());
    }

    #[test]
    fn models_compare_by_value() {
        let a = ProjectLink { url: "u".into(), description: "d".into(), language: Some("Rust".into()) };
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(Contact { name: "linkedin".into(), url: "x".into() },
                   Contact { name: "linkedin".into(), url: "x".into() });
    }
}
