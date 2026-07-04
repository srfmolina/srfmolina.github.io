//! Serde DTOs mirroring the raw `index.json` shape. Krocy's data-layer entities.

use std::collections::BTreeMap;

use serde::Deserialize;

/// The whole `index.json` document.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LinksDto {
    #[serde(default)]
    pub github: Vec<LinkDto>,
    #[serde(default)]
    pub codeberg: Vec<LinkDto>,
    #[serde(default)]
    pub extra: BTreeMap<String, String>,
}

/// A single project/link entry. `language` is absent for non-code repos.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LinkDto {
    pub link: String,
    pub description: String,
    #[serde(default)]
    pub language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_real_shape_with_optional_language_and_extra() {
        let json = r#"{
            "github": [
                { "link": "https://github.com/srfmolina/mod-comp", "description": "models", "language": "Rust" },
                { "link": "https://github.com/srfmolina/srfmolina", "description": "hello" }
            ],
            "codeberg": [
                { "link": "https://codeberg.org/srfmolina/pages", "description": "portfolio", "language": "Rust" }
            ],
            "extra": { "linkedin": "https://linkedin.com/in/srfmolina" }
        }"#;

        let dto: LinksDto = serde_json::from_str(json).expect("valid json");

        assert_eq!(dto.github.len(), 2);
        assert_eq!(dto.github[0].language.as_deref(), Some("Rust"));
        assert_eq!(dto.github[1].language, None);
        assert_eq!(dto.codeberg.len(), 1);
        assert_eq!(dto.extra.get("linkedin").map(String::as_str), Some("https://linkedin.com/in/srfmolina"));
    }

    #[test]
    fn missing_arrays_default_to_empty() {
        let dto: LinksDto = serde_json::from_str("{}").expect("valid json");
        assert!(dto.github.is_empty());
        assert!(dto.codeberg.is_empty());
        assert!(dto.extra.is_empty());
    }
}
