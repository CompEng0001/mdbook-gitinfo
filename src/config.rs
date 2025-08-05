//! Configuration module for the `mdbook-gitinfo` preprocessor.
//!
//! This module provides the [`GitInfoConfig`] struct, which represents user-defined options
//! from the `[preprocessor.gitinfo]` table in `book.toml`, and a function to load this configuration
//! from the `mdbook` context.

use mdbook::errors::Error;
use mdbook::preprocess::PreprocessorContext;
use serde::Deserialize;

/// Represents the user-defined configuration options under `[preprocessor.gitinfo]`
/// in `book.toml`.
///
/// Each field is optional and has a default fallback within the main logic of the preprocessor.
#[derive(Debug, Deserialize)]
pub struct GitInfoConfig {
    /// The formatting style of the git data (not currently used in logic).
    pub format: Option<String>,

    /// Template string allowing users to define how git metadata is rendered.
    /// Supported placeholders: `{{hash}}`, `{{long}}`, `{{tag}}`, `{{date}}`, `{{sep}}`.
    pub template: Option<String>,

    /// Font size for the rendered git information footer.
    /// Default: `"0.8em"`.
    #[serde(rename = "font-size")]
    pub font_size: Option<String>,

    /// Separator string inserted between elements (e.g., between date and hash).
    /// Default: `" • "`.
    pub separator: Option<String>,

    /// Format string for the date component using `chrono` formatting syntax.
    /// Default: `"%Y-%m-%d"`.
    #[serde(rename = "date-format")]
    pub date_format: Option<String>,

    /// Format string for the time component using `chrono` formatting syntax.
    /// Default: `"%H:%M:%S"`.
    #[serde(rename = "time-format")]
    pub time_format: Option<String>,

    /// Set the branch from where to get the commit history from
    /// Default: "main"
    pub branch: Option<String>,
}

/// Load and deserialize the `[preprocessor.gitinfo]` table from `book.toml`.
///
/// # Arguments
///
/// * `ctx` - The `PreprocessorContext` provided by `mdbook`, containing global configuration.
///
/// # Errors
///
/// Returns an [`Error`] if the section is missing or cannot be parsed.
///
/// # Example
///
/// ```no_run
/// let cfg = load_config(&ctx)?;
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
