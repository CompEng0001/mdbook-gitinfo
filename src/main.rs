//! `mdbook-gitinfo`: A preprocessor for `mdBook` that injects Git commit metadata into book content.
//!
//! This preprocessor extracts the latest commit hash, tag, and timestamp from the repository
//! and renders it into each chapter, typically for provenance or change-tracking purposes.

mod git;
mod config;

use crate::config::load_config;
use chrono::{DateTime, Local};
use clap::{arg, command, ArgMatches, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::{io, process};

/// The `GitInfo` struct implements the `Preprocessor` trait for injecting Git metadata.
///
/// It reads settings from `[preprocessor.gitinfo]` in `book.toml`, including optional fields like
/// `template`, `font-size`, `separator`, `date-format`, and `time-format`.
pub struct GitInfo;

impl GitInfo {
    /// Creates a new `GitInfo` preprocessor instance.
    pub fn new() -> Self {
        GitInfo
    }
}

impl Preprocessor for GitInfo {
    fn name(&self) -> &str {
        "gitinfo"
    }

    /// Injects rendered Git metadata into each chapter of the book.
    ///
    /// Metadata is extracted using Git commands, and rendered based on a user-specified or
    /// default template. The result is inserted at the bottom of each chapter wrapped in a styled `<footer>`.
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = load_config(ctx)?;

        // Git metadata extraction
        let hash = git::get_git_output(["rev-parse", "--short", "HEAD"], &ctx.root)?;
        let long_hash = git::get_git_output(["rev-parse", "HEAD"], &ctx.root)?;
        let tag = git::get_git_output(["describe", "--tags", "--always"], &ctx.root)?;
        let raw_date = git::get_git_output(
            ["log", "-1", "--format=%cd", "--date=iso-strict"],
            &ctx.root,
        )?;

        // Configurable rendering options
        let template = cfg.template.unwrap_or_else(|| "{{date}}{{sep}}commit: {{hash}}".to_string());
        let font_size = cfg.font_size.unwrap_or_else(|| "0.8em".to_string());
        let separator = cfg.separator.unwrap_or_else(|| " • ".to_string());
        let date_format = cfg.date_format.unwrap_or_else(|| "%Y-%m-%d".to_string());
        let time_format = cfg.time_format.unwrap_or_else(|| "%H:%M:%S".to_string());

        // Attempt to parse and format the commit timestamp
        let formatted_date = match DateTime::parse_from_rfc3339(&raw_date) {
            Ok(dt) => format!(
                "{} {}",
                dt.with_timezone(&Local).format(&date_format),
                dt.with_timezone(&Local).format(&time_format)
            ),
            Err(_) => raw_date.clone(),
        };

        // Render the template
        let rendered = template
            .replace("{{hash}}", &hash)
            .replace("{{long}}", &long_hash)
            .replace("{{tag}}", &tag)
            .replace("{{date}}", &formatted_date)
            .replace("{{sep}}", &separator);

        // Inline style for visibility control
        let style = format!(
            "font-size:{};padding:4px;margin:0.5em 0;text-align:right;display:block;",
            font_size
        );

        let decorated = format!(
            "<footer><span class=\"gitinfo-footer\" style=\"{}\">{}</span></footer>",
            style, rendered
        );

        // Inject footer into all chapters
        for section in &mut book.sections {
            if let BookItem::Chapter(ch) = section {
                ch.content.push_str("\n");
                ch.content.push_str(&decorated);
            }
        }

        Ok(book)
    }

    /// Indicates this preprocessor supports only the `html` renderer.
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// Handles the normal preprocessing workflow for `mdbook build` and `mdbook serve`.
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The '{}' plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}

/// Handles the `supports` subcommand invoked by `mdbook`.
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Renderer argument required");

    if pre.supports_renderer(renderer) {
        process::exit(0);
    }
    process::exit(1);
}

/// Entry point for the `mdbook-gitinfo` binary.
///
/// Supports two modes:
/// - `supports <renderer>`: Called by `mdbook` to check compatibility.
/// - `stdin -> stdout`: Standard preprocessor input/output for mdbook pipelines.
fn main() {
    let matches = command!("mdbook-gitinfo")
        .about("An mdbook preprocessor that injects Git metadata into the book")
        .subcommand(
            Command::new("supports")
                .arg(arg!(<renderer> "The renderer name to check support for"))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    let pre = GitInfo::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&pre, sub_args);
    }

    if let Err(e) = handle_preprocessing(&pre) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

pub fn render_template(
    template: &str,
    hash: &str,
    long_hash: &str,
    tag: &str,
    date: &str,
    sep: &str,
) -> String {
    template
        .replace("{{hash}}", hash)
        .replace("{{long}}", long_hash)
        .replace("{{tag}}", tag)
        .replace("{{date}}", date)
        .replace("{{sep}}", sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_basic() {
        let output = render_template(
            "Commit: {{hash}} on {{date}}",
            "abcd123",
            "",
            "",
            "2025-06-24",
            "|",
        );
        assert_eq!(output, "Commit: abcd123 on 2025-06-24");
    }
}
