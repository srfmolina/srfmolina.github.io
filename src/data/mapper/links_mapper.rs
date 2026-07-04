//! Maps the JSON DTOs onto domain models (Krocy's `data.mapper`).

use crate::data::entity::links::{LinkDto, LinksDto};
use crate::domain::model::links::{Contact, Links, ProjectLink};

impl From<LinkDto> for ProjectLink {
    fn from(dto: LinkDto) -> Self {
        ProjectLink {
            url: dto.link,
            description: dto.description,
            language: dto.language,
        }
    }
}

/// Convert the whole DTO document to the domain aggregate.
/// `extra` becomes contacts in key-sorted order (`BTreeMap` is ordered).
pub fn map_links(dto: LinksDto) -> Links {
    Links {
        github: dto.github.into_iter().map(ProjectLink::from).collect(),
        codeberg: dto.codeberg.into_iter().map(ProjectLink::from).collect(),
        contacts: dto
            .extra
            .into_iter()
            .map(|(name, url)| Contact { name, url })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn maps_link_field_to_url_and_keeps_optional_language() {
        let dto = LinkDto { link: "https://x".into(), description: "d".into(), language: Some("Rust".into()) };
        let project: ProjectLink = dto.into();
        assert_eq!(project, ProjectLink { url: "https://x".into(), description: "d".into(), language: Some("Rust".into()) });
    }

    #[test]
    fn maps_extra_map_into_sorted_contacts() {
        let mut extra = BTreeMap::new();
        extra.insert("linkedin".to_string(), "https://li".to_string());
        extra.insert("bluesky".to_string(), "https://bsky".to_string());
        let dto = LinksDto { github: vec![], codeberg: vec![], extra };

        let links = map_links(dto);

        // BTreeMap iteration is key-sorted: bluesky before linkedin.
        assert_eq!(links.contacts, vec![
            Contact { name: "bluesky".into(), url: "https://bsky".into() },
            Contact { name: "linkedin".into(), url: "https://li".into() },
        ]);
    }

    #[test]
    fn maps_full_document() {
        let dto = LinksDto {
            github: vec![LinkDto { link: "g".into(), description: "gd".into(), language: None }],
            codeberg: vec![LinkDto { link: "c".into(), description: "cd".into(), language: Some("Rust".into()) }],
            extra: BTreeMap::new(),
        };

        let links = map_links(dto);

        assert_eq!(links.github, vec![ProjectLink { url: "g".into(), description: "gd".into(), language: None }]);
        assert_eq!(links.codeberg, vec![ProjectLink { url: "c".into(), description: "cd".into(), language: Some("Rust".into()) }]);
    }
}
