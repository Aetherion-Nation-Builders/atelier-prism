//! The Nation and project links (shown in Help ▸, the About dialog, the start screen and the
//! header's Nation button). Command ids keep their upstream names (`help.discord` opens the Nation,
//! `help.website` the Atelier suite).

pub const APP_NAME: &str = "Atelier Prism";
pub const APP_SLUG: &str = "prism";
/// The Aetherion Nation.
pub const DISCORD: &str = "https://www.aetherionnnation.com";
/// Atelier, the Nation's creative suite.
pub const WEBSITE: &str = "https://www.aetherionnnation.com/atelier";
pub const APP_PAGE: &str = "https://www.aetherionnnation.com/atelier/prism";
pub const GITHUB: &str = "https://github.com/Aetherion-Nation-Builders/atelier-prism";
pub const ISSUES: &str = "https://github.com/Aetherion-Nation-Builders/atelier-prism/issues";

/// The other Atelier apps: (name, slug on the Nation site and in `atelier-<slug>`).
pub const SIBLINGS: &[(&str, &str)] =
    &[("Pigment", "pigment"), ("Quill", "quill"), ("Reel", "reel"), ("Frequency", "frequency"), ("Lens", "lens"), ("Seal", "seal"), ("Press", "press")];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/Aetherion-Nation-Builders/atelier-{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("{WEBSITE}/{slug}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_atelier_scheme() {
        assert!(WEBSITE.starts_with(DISCORD));
        assert_eq!(APP_PAGE, page_of(APP_SLUG));
        assert_eq!(GITHUB, github_of(APP_SLUG));
        assert!(ISSUES.starts_with(GITHUB));
        for (name, slug) in SIBLINGS {
            assert!(!name.is_empty() && page_of(slug).starts_with("https://") && github_of(slug).ends_with(slug));
        }
    }
}
