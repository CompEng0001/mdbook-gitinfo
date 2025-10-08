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
use serde::Deserialize;

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
    /// - `{{tag}}` → nearest tag
    /// - `{{date}}` → commit date
    /// - `{{sep}}` → separator string
    pub template: Option<String>,

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

    /// Git branch from which to retrieve commit history.
    ///
    /// Default: `"main"`.
    pub branch: Option<String>,

    /// CSS option to align footer 
    ///
    /// Options: "left | center | right" 
    /// Default: `"center"`.
    pub align: Option<String>,

    /// CSS option to adjust margin between body and footer 
    ///
    /// Options: should use em 
    /// Default: `"center"`.
    #[serde(rename = "margin-top")]
    pub margin_top: Option<String>,

    /// CSS option provides a hyperlink to the respective branch and commit  
    /// in the footer
    ///
    /// Options: "true | false" 
    /// Default: `false`.
    pub hyperlink: Option<bool>,
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
    use std::collections::BTreeMap;
    use toml::Value;

    fn make_context_with_toml(toml: &str) -> PreprocessorContext {
        let parsed: toml::Value = toml::from_str(toml).unwrap();
        let mut config = Config::default();
        config.set("preprocessor.gitinfo", parsed);
        PreprocessorContext {
            config,
            ..Default::default()
        }
    }

    #[test]
    fn parses_valid_config() {
        let ctx = make_context_with_toml(
            r#"
                template = "Commit: {{hash}}"
                separator = " | "
                font-size = "1em"
            "#,
        );

        let config = load_config(&ctx).unwrap();
        assert_eq!(config.template.unwrap(), "Commit: {{hash}}");
        assert_eq!(config.separator.unwrap(), " | ");
    }
}
