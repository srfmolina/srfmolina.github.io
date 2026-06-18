//! Typed, nested string tree for the UI — one `Texts` root per language.
//!
//! Mirrors the author's prior `translations.rs`: a single root struct nesting
//! grouped sub-structs, resolved from two complete `&'static` constants. Every
//! field is `&'static str`, so the project/tech lists live in `const` arrays
//! and the whole tree is baked into the binary. Forgetting a field in one
//! language is a *compile* error.

use super::Language;

/// A "favourite project" card. Identity (`no`/`name`/`lang`) and localized
/// prose (`tag`/`desc`) all live here as `&'static str`.
#[derive(Clone, PartialEq)]
pub struct Project {
    pub no: &'static str,
    pub name: &'static str,
    pub lang: &'static str,
    pub tag: &'static str,
    pub desc: &'static str,
}

/// A technology card. `name` is a proper noun; `cat` is localized.
#[derive(Clone, PartialEq)]
pub struct Tech {
    pub name: &'static str,
    pub cat: &'static str,
}

/// Header navigation labels.
pub struct NavTexts {
    pub projects: &'static str,
    pub stack: &'static str,
}

/// Theme-toggle labels (the word shown next to the icon).
pub struct ThemeTexts {
    pub light: &'static str,
    pub dark: &'static str,
}

/// Footer copy.
pub struct FooterTexts {
    pub meta: &'static str,
}

/// Hero-section copy.
pub struct HeroTexts {
    pub eyebrow: &'static str,
    pub lead_accent: &'static str,
    pub lead_rest: &'static str,
    pub para2_prefix: &'static str,
    pub para2_suffix: &'static str,
    pub chip_role: &'static str,
    pub chip_platforms: &'static str,
    pub portrait_placeholder: &'static str,
}

/// Everything the home screen renders.
pub struct HomeTexts {
    pub hero: HeroTexts,
    pub projects_title: &'static str,
    pub projects_note: &'static str,
    pub stack_title: &'static str,
    pub stack_note: &'static str,
    pub projects: &'static [Project],
    pub techs: &'static [Tech],
}

/// The localization root. One complete instance per language.
pub struct Texts {
    pub lang: &'static str,
    pub nav: NavTexts,
    pub theme: ThemeTexts,
    pub home: HomeTexts,
    pub footer: FooterTexts,
}

impl Texts {
    /// Resolve the full tree for `lang`. Returns a reference to the matching
    /// `&'static` constant.
    pub const fn for_language(lang: Language) -> &'static Texts {
        match lang {
            Language::English => &ENGLISH_TEXTS,
            Language::Spanish => &SPANISH_TEXTS,
        }
    }
}

pub const ENGLISH_TEXTS: Texts = Texts {
    lang: "en",
    nav: NavTexts { projects: "Projects", stack: "Stack" },
    theme: ThemeTexts { light: "LIGHT", dark: "DARK" },
    home: HomeTexts {
        hero: HeroTexts {
            eyebrow: "Computer Scientist",
            lead_accent: "Passionate about science and computer science.",
            lead_rest: " I studied the most formal and scientific specialisation of my degree — rigorous, comprehensive university training.",
            para2_prefix: "Today I'm a mobile developer at ",
            para2_suffix: ", building native mobile apps for Android with Jetpack Compose, and multiplatform with Rust (Dioxus).",
            chip_role: "Full Stack @ NTT Data",
            chip_platforms: "Android native and multiplatform",
            portrait_placeholder: "Add your photo",
        },
        projects_title: "Favourite projects",
        projects_note: "things I loved building",
        stack_title: "Technologies I know",
        stack_note: "my favourite tools",
        projects: &[
            Project { no: "01", name: "Krocy", lang: "Kotlin", tag: "Kotlin Multiplatform", desc: "A Grocy client built with Kotlin Multiplatform — all shared between every screen." },
            Project { no: "02", name: "SFDataStructures", lang: "Haskell", tag: "Functional", desc: "A library of classic data structures implemented from scratch in Haskell, with an emphasis on purity and correctness." },
            Project { no: "03", name: "Supertres", lang: "Rust", tag: "Game", desc: "A super tic-tac-toe game written in Rust — recursive boards, tight rules, zero garbage collection." },
        ],
        techs: &[
            Tech { name: "Kotlin", cat: "Mobile" },
            Tech { name: "Rust", cat: "Systems" },
            Tech { name: "Haskell", cat: "Functional" },
            Tech { name: "Python", cat: "Scripting" },
            Tech { name: "Qiskit", cat: "Quantum" },
            Tech { name: "CUDA", cat: "GPU" },
        ],
    },
    footer: FooterTexts { meta: "© 2026 · built with care" },
};

pub const SPANISH_TEXTS: Texts = Texts {
    lang: "es",
    nav: NavTexts { projects: "Proyectos", stack: "Stack" },
    theme: ThemeTexts { light: "CLARO", dark: "OSCURO" },
    home: HomeTexts {
        hero: HeroTexts {
            eyebrow: "Ingeniero informático",
            lead_accent: "Apasionado por la ciencia y la informática.",
            lead_rest: " Estudié la especialización más formal y científica de mi carrera: una formación universitaria rigurosa y completa.",
            para2_prefix: "Actualmente soy desarrollador mobile en ",
            para2_suffix: ", donde desarrollo apps móviles nativas para Android con Jetpack Compose, y multiplataforma con Rust (Dioxus).",
            chip_role: "Full Stack @ NTT Data",
            chip_platforms: "Android nativo y multiplataforma",
            portrait_placeholder: "Añade tu foto",
        },
        projects_title: "Proyectos favoritos",
        projects_note: "cosas que disfruté construyendo",
        stack_title: "Tecnologías que conozco",
        stack_note: "mis herramientas favoritas",
        projects: &[
            Project { no: "01", name: "Krocy", lang: "Kotlin", tag: "Kotlin Multiplatform", desc: "Un cliente de Grocy hecho con Kotlin Multiplatform: todo compartido entre cada plataforma." },
            Project { no: "02", name: "SFDataStructures", lang: "Haskell", tag: "Funcional", desc: "Una biblioteca de estructuras de datos clásicas implementadas desde cero en Haskell, con énfasis en la pureza y la corrección." },
            Project { no: "03", name: "Supertres", lang: "Rust", tag: "Juego", desc: "Un juego de tres en raya recursivo escrito en Rust: tableros anidados, reglas estrictas y cero recolección de basura." },
        ],
        techs: &[
            Tech { name: "Kotlin", cat: "Móvil" },
            Tech { name: "Rust", cat: "Sistemas" },
            Tech { name: "Haskell", cat: "Funcional" },
            Tech { name: "Python", cat: "Scripting" },
            Tech { name: "Qiskit", cat: "Cuántica" },
            Tech { name: "CUDA", cat: "GPU" },
        ],
    },
    footer: FooterTexts { meta: "© 2026 · hecho con cariño" },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_language_resolves_each_constant() {
        assert_eq!(Texts::for_language(Language::English).lang, "en");
        assert_eq!(Texts::for_language(Language::Spanish).lang, "es");
    }

    #[test]
    fn chrome_and_home_strings_differ_across_languages() {
        let en = Texts::for_language(Language::English);
        let es = Texts::for_language(Language::Spanish);
        assert_ne!(en.nav.projects, es.nav.projects);
        assert_ne!(en.home.hero.eyebrow, es.home.hero.eyebrow);
        assert_ne!(en.footer.meta, es.footer.meta);
    }

    #[test]
    fn projects_keep_identity_but_localize_prose() {
        let en = Texts::for_language(Language::English).home.projects;
        let es = Texts::for_language(Language::Spanish).home.projects;
        assert_eq!(en.len(), 3);
        let en_names: Vec<&str> = en.iter().map(|p| p.name).collect();
        let es_names: Vec<&str> = es.iter().map(|p| p.name).collect();
        assert_eq!(en_names, ["Krocy", "SFDataStructures", "Supertres"]);
        assert_eq!(en_names, es_names);
        assert_ne!(en[0].desc, es[0].desc);
    }

    #[test]
    fn techs_keep_names_but_localize_categories() {
        let en = Texts::for_language(Language::English).home.techs;
        let es = Texts::for_language(Language::Spanish).home.techs;
        assert_eq!(en.len(), 6);
        let en_names: Vec<&str> = en.iter().map(|t| t.name).collect();
        assert_eq!(en_names, ["Kotlin", "Rust", "Haskell", "Python", "Qiskit", "CUDA"]);
        assert_ne!(en[0].cat, es[0].cat); // Mobile vs Móvil
    }
}
