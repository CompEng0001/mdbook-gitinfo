//! Configuration module for the `mdbook-gitinfo` preprocessor.
//!
//! This module defines [`GitInfoConfig`], the structure that holds all
//! user-defined configuration options from the `[preprocessor.gitinfo]`
//! section in `book.toml`. It also provides [`load_config`] to deserialize
//! these values into the struct for use by the preprocessor.
//!
//! # Example `book.toml`
//!
//! ```toml
//! [preprocessor.gitinfo]
//! template   = "Date: {{date}} • Commit: {{hash}}"
//! font-size  = "0.8em"
//! separator  = " | "
//! date-format = "%Y-%m-%d"
//! time-format = "%H:%M:%S"
//! branch      = "main"
//! ```

use mdbook::errors::Error;
use mdbook::preprocess::PreprocessorContext;
use std::collections::BTreeSet;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct MessageConfig {
    /// Header message template
    pub header: Option<String>,
    /// Footer message template
    pub footer: Option<String>,
    /// Default for both (used if header/footer not set)
    pub both: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MarginSetting {
    /// "1em"
    One(String),
    /// ["top", "right", "bottom", "left"] — supports 1–4 entries like CSS shorthand
    Quad(Vec<String>),
    /// { top = "...", right = "...", bottom = "...", left = "..." }
    Sides {
        top: Option<String>,
        right: Option<String>,
        bottom: Option<String>,
        left: Option<String>,
    },
}

impl Default for MarginSetting {
    fn default() -> Self { MarginSetting::One("0".to_string()) }
}

#[derive(Debug, Deserialize, Default)]
pub struct MarginConfig {
    pub header: Option<MarginSetting>,
    pub footer: Option<MarginSetting>,
    pub both:   Option<MarginSetting>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AlignSetting {
    /// Legacy: align = "center"
    One(String),
    /// New: align = { header = "...", footer = "...", both = "..." }
    Split {
        header: Option<String>,
        footer: Option<String>,
        both:   Option<String>,
    },
}

impl Default for AlignSetting {
    fn default() -> Self { AlignSetting::One("center".to_string()) }
}

/// Represents the user-defined configuration options under `[preprocessor.gitinfo]`
/// in `book.toml`.
///
/// Each field is optional; defaults are handled in the preprocessor logic.
/// The configuration allows users to control how commit metadata is formatted
/// and rendered in the generated book.
#[derive(Debug, Deserialize,Default)]
pub struct GitInfoConfig {
    /// Gate to turn the preprocessor on/off without removing the section.
    /// Default: true (when omitted).
    pub enable: Option<bool>,

    /// The formatting style of the git data (currently unused, reserved for future use).
    pub format: Option<String>,
    
    /// Template string defining how git metadata is rendered.
    ///
    /// Supported placeholders:
    /// - `{{hash}}` → short commit hash
    /// - `{{long}}` → full commit hash
    /// - `{{tag}}` → lastest tag or user defined
    /// - `{{date}}` → commit date
    /// - `{{sep}}` → separator string
    /// (Deprecated) Old single template. If present, used as a fallback for footer_message.
    pub template: Option<String>,
    
    // Placement switches
    pub header: Option<bool>,
    pub footer: Option<bool>,

    /// Message templates in a table: message.header/message.footer/message.both
    pub message: Option<MessageConfig>,

    /// CSS font size for the rendered footer text.
    ///
    /// Default: `"0.8em"`.
    #[serde(rename = "font-size")]
    pub font_size: Option<String>,

    /// String separator inserted between elements (e.g., date and hash).
    ///
    /// Default: `" • "`.
    pub separator: Option<String>,

    /// Format string for the date component.
    ///
    /// Uses the [`chrono`] crate formatting syntax.
    /// Default: `"%Y-%m-%d"`.
    #[serde(rename = "date-format")]
    pub date_format: Option<String>,

    /// Format string for the time component.
    ///
    /// Uses the [`chrono`] crate formatting syntax.
    /// Default: `"%H:%M:%S"`.
    #[serde(rename = "time-format")]
    pub time_format: Option<String>,


    pub timezone: Option<String>,        // "local" | "utc" | "source" | "fixed:+01:00" | "rfc3339"
    pub datetime_format: Option<String>, // optional: if set, overrides date/time format join
    pub show_offset: Option<bool>,       // optional: if true and no %z/%:z/%Z, append %:z
    
    /// Git branch from which to retrieve commit history.
    ///
    /// Default: `"main"`.
    pub branch: Option<String>,

    /// Flexible align
    /// - align = "center"
    /// - align.header = "left", align.footer = "right"
    /// - [preprocessor.gitinfo.align] both = "center"
    pub align: Option<AlignSetting>,

    /// CSS option to adjust margin between body and footer 
    pub margin: Option<MarginConfig>,

    // explicit tag override (if set, use this instead of auto-detect)
    pub tag: Option<String>,

    /// CSS option provides a hyperlink to the respective branch and commit  
    /// in the footer
    ///
    /// Options: "true | false" 
    /// Default: `false`.
    pub hyperlink: Option<bool>,

    /// Git Contributor switch
    pub contributors: Option<bool>,

    /// Optional title for the contributors block.
    ///
    /// Default: "Student Contributors:"
    #[serde(rename = "contributor-title")]
    pub contributor_title: Option<String>,

    /// List of contributor author names to exclude.
    ///
    /// Matches against the git author name (treated as GitHub username).
    ///
    /// Example:
    /// exclude-contributors = ["github-actions[bot]", "template-author"]
    #[serde(rename = "exclude-contributors")]
    pub exclude_contributors: Option<Vec<String>>,
}

/// Load and deserialize the `[preprocessor.gitinfo]` table from `book.toml`.
///
/// # Arguments
///
/// * `ctx` — The [`PreprocessorContext`] provided by `mdbook`, containing
///   the configuration tree.
///
/// # Errors
///
/// Returns an [`Error`] if the section is missing or cannot be parsed.
///
/// # Examples
///
/// ```no_run
/// use mdbook::preprocess::PreprocessorContext;
/// use mdbook_gitinfo::config::load_config;
///
/// # fn example(ctx: &PreprocessorContext) -> Result<(), mdbook::errors::Error> {
/// let cfg = load_config(ctx)?;
/// if let Some(template) = cfg.template {
///     println!("Using template: {}", template);
/// }
/// # Ok(())
/// # }
/// ```
pub fn load_config(ctx: &PreprocessorContext) -> Result<GitInfoConfig, Error> {
    ctx.config
        .get("preprocessor.gitinfo")
        .and_then(|t| t.clone().try_into().ok())
        .ok_or_else(|| Error::msg("Missing or invalid [preprocessor.gitinfo] config"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdbook::Config;

    fn ctx(toml: &str) -> mdbook::preprocess::PreprocessorContext {
        let parsed: toml::Value = toml::from_str(toml).unwrap();
        let mut config = Config::default();
        config.set("preprocessor.gitinfo", parsed);
        mdbook::preprocess::PreprocessorContext { config, ..Default::default() }
    }

    #[test]
    fn parses_legacy_align() {
        let c = load_config(&ctx(r#"align = "left""#)).unwrap();
        match c.align.unwrap() {
            AlignSetting::One(s) => assert_eq!(s, "left"),
            _ => panic!("expected One"),
        }
    }

    #[test]
    fn parses_split_align() {
        let c = load_config(&ctx(r#"
            [align]
            both = "center"
            header = "left"
        "#)).unwrap();
        match c.align.unwrap() {
            AlignSetting::Split { header, footer, both } => {
                assert_eq!(header.as_deref(), Some("left"));
                assert_eq!(footer, None);
                assert_eq!(both.as_deref(), Some("center"));
            }
            _ => panic!("expected Split"),
        }
    }

    #[test]
    fn message_resolution_parses() {
        let c = load_config(&ctx(r#"
            [message]
            both = "D: {{date}}"
            header = "H: {{date}}"
        "#)).unwrap();
        assert_eq!(c.message.unwrap().header.unwrap(), "H: {{date}}");
    }
}