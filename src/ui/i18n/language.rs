//! The set of supported languages and their persistence helpers.
//!
//! Modeled on the existing `Theme` enum: `code`/`from_code` are the
//! `localStorage` (de)serialization pair (the `data_attr`/`from_stored` analog).

/// A supported UI language. `English` is the default (first-visit) language.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    /// The value persisted in `localStorage` (BCP-47-ish short code).
    pub const fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }

    /// Parse a persisted code back into a language (`None` if unrecognized).
    pub fn from_code(value: &str) -> Option<Language> {
        match value {
            "en" => Some(Language::English),
            "es" => Some(Language::Spanish),
            _ => None,
        }
    }

    /// The language's own native name, shown in the switcher dropdown.
    pub const fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
        }
    }

    /// Every language, in switcher display order. Adding a variant here (and to
    /// the `match`es above) makes it appear in the dropdown automatically.
    pub fn all() -> &'static [Language] {
        &[Language::English, Language::Spanish]
    }

    /// The other language — reserved for a future two-state toggle.
    pub fn toggled(self) -> Language {
        match self {
            Language::English => Language::Spanish,
            Language::Spanish => Language::English,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trips_for_every_language() {
        for &lang in Language::all() {
            assert_eq!(Language::from_code(lang.code()), Some(lang));
        }
    }

    #[test]
    fn from_code_rejects_unknown_values() {
        assert_eq!(Language::from_code("fr"), None);
        assert_eq!(Language::from_code(""), None);
    }

    #[test]
    fn default_language_is_english() {
        assert_eq!(Language::default(), Language::English);
    }

    #[test]
    fn all_lists_every_variant() {
        assert_eq!(Language::all(), &[Language::English, Language::Spanish]);
    }

    #[test]
    fn label_is_the_native_name() {
        assert_eq!(Language::English.label(), "English");
        assert_eq!(Language::Spanish.label(), "Español");
    }

    #[test]
    fn toggled_flips_between_the_two() {
        assert_eq!(Language::English.toggled(), Language::Spanish);
        assert_eq!(Language::Spanish.toggled(), Language::English);
    }
}
