//! `mdbook-gitinfo`: A preprocessor for `mdBook` that injects Git commit metadata into book content.
//!
//! This preprocessor extracts the latest commit hash, tag, and timestamp from the repository
//! and renders it into each chapter, typically for provenance or change-tracking purposes.

use mdbook_gitinfo::config::load_config;
use mdbook_gitinfo::git;

use chrono::{DateTime, Local};
use clap::{ArgMatches, Command, arg, command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::path::PathBuf;
use std::{io, process};

/// The `GitInfo` struct implements the `Preprocessor` trait for injecting Git metadata.
///
/// It reads settings from `[preprocessor.gitinfo]` in `book.toml`, including optional fields like
/// `template`, `font-size`, `separator`, `date-format`, `time-format` and `branch`.
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
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // If the config table is absent or invalid, fall back to defaults.
        let cfg = load_config(ctx).unwrap_or_default();

        let enabled = cfg.enable.unwrap_or(true);
        if !enabled {
            return Ok(book);
        }

        // Git metadata extraction
        let template = cfg
            .template
            .unwrap_or_else(|| "{{date}}{{sep}}commit: {{hash}}".to_string());
        let font_size = cfg.font_size.unwrap_or_else(|| "0.8em".to_string());
        let text_align = cfg.align.unwrap_or_else(|| "center".to_string());
        let margin_top = cfg.margin_top.unwrap_or_else(|| "2em".to_string());
        let separator = cfg.separator.unwrap_or_else(|| " • ".to_string());
        let date_format = cfg.date_format.unwrap_or_else(|| "%Y-%m-%d".to_string());
        let time_format = cfg.time_format.unwrap_or_else(|| "%H:%M:%S".to_string());
        let mut branch = cfg.branch.unwrap_or_else(|| "main".to_string());
        let hyperlink = cfg.hyperlink.unwrap_or(false);
        let repo_base = if hyperlink { resolve_repo_base(&ctx.root) } else { None };

        // Verify the branch exists
        if !git::verify_branch(&branch, &ctx.root) {
            eprintln!("Warning: Branch '{}' not found, falling back to 'main'", branch);
            branch = "main".to_string();
        }

        let content_dir = ctx.config.book.src.clone();

        book.for_each_mut(|item| {
            decorate_chapters(item, &|ch| {
                if let Some(path) = &ch.path {
                    // Get relative path Git log
                    let full_path = PathBuf::from(&content_dir).join(path);
                    let path_str = full_path.to_string_lossy().replace('\\', "/");

                    // Configurable rendering options
                    let short_hash = git::get_git_output(
                        ["log", "-1", "--format=%h", &format!("{branch}"), "--", &path_str],
                        &ctx.root,
                    )
                    .unwrap_or_default();
                    let long_hash = git::get_git_output(
                        ["log", "-1", "--format=%H", &format!("{branch}"), "--", &path_str],
                        &ctx.root,
                    )
                    .unwrap_or_default();
                    let tag = git::get_git_output(
                        ["describe", "--tags", "--always", "--", &path_str],
                        &ctx.root,
                    )
                    .unwrap_or_default();
                    let raw_date = git::get_git_output(
                        [
                            "log",
                            "-1",
                            "--format=%cd",
                            "--date=iso-strict",
                            "--",
                            &path_str,
                        ],
                        &ctx.root,
                    )
                    .unwrap_or_else(|_| "unknown".to_string());

                    // Attempt to parse and format the commit timestamp
                    let formatted_date = DateTime::parse_from_rfc3339(&raw_date)
                        .map(|dt| {
                            format!(
                                "{} {}",
                                dt.with_timezone(&Local).format(&date_format),
                                dt.with_timezone(&Local).format(&time_format)
                            )
                        })
                        .unwrap_or_else(|_| "unknown".to_string());

                    let (hash_disp, branch_disp) = if let (true, Some(base)) = (hyperlink, repo_base.as_ref()) {
                        let commit_url = format!("{}/commit/{}", base, long_hash);
                        let branch_url = format!("{}/tree/{}", base, branch);
                        (
                            format!(r#"<a href="{}">{}</a>"#, commit_url, short_hash),
                            format!(r#"<a href="{}">{}</a>"#, branch_url, branch),
                        )
                    } else {
                        (short_hash.clone(), branch.clone())
                    };

                    // Render the template
                    let rendered = render_template(
                        &template,
                        &hash_disp,
                        &long_hash,
                        &tag,
                        &formatted_date,
                        &separator,
                        &branch_disp,
                    );

                    // Inline style for visibility control
                    let style = format!(
                        "font-size:{};padding:4px;margin-top:{};text-align:{};display:block;",
                        font_size, margin_top, text_align
                    );

                    let decorated = format!(
                        "<footer class=\"gitinfo-footer\" style=\"{}\">{}</footer>",
                        style, rendered
                    );

                    // Inject footer into all chapters/subchapters etc
                    if !ch.content.contains(&decorated) {
                        ch.content.push_str("\n");
                        ch.content.push_str(&decorated);
                    }
                }
            });
        });

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
            "Warning: The '{}' plugin was built against version {} of mdbook, but we're being called from version {}",
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

// normalises repo url from ssh to https
fn normalise_repo_base(url: &str) -> String {
    let u = url.trim().trim_end_matches(".git").to_string();
    if let Some(rest) = u.strip_prefix("git@github.com:") {
        return format!("https://github.com/{rest}");
    }
    if let Some(rest) = u.strip_prefix("ssh://git@github.com/") {
        return format!("https://github.com/{rest}");
    }
    u
}

// Depending environment the repo name is and url is retrieved
fn resolve_repo_base(ctx_root: &std::path::Path) -> Option<String> {
    if let (Ok(server), Ok(repo)) = (std::env::var("GITHUB_SERVER_URL"), std::env::var("GITHUB_REPOSITORY")) {
        return Some(format!("{}/{}", server.trim_end_matches('/'), repo));
    }

    if let (Ok(server), Ok(path)) = (std::env::var("CI_SERVER_URL"), std::env::var("CI_PROJECT_PATH")) {
        return Some(format!("{}/{}", server.trim_end_matches('/'), path));
    }

    if let Ok(http_origin) = std::env::var("BITBUCKET_GIT_HTTP_ORIGIN") {
        return Some(normalise_repo_base(&http_origin));
    }
    if let Ok(full) = std::env::var("BITBUCKET_REPO_FULL_NAME") {
        return Some(format!("https://bitbucket.org/{}", full));
    }

    if let Ok(remote) = mdbook_gitinfo::git::get_git_output(["config", "--get", "remote.origin.url"], ctx_root) {
        return Some(normalise_repo_base(&remote));
    }

    None
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
    branch: &str,
) -> String {
    template
        .replace("{{hash}}", hash)
        .replace("{{long}}", long_hash)
        .replace("{{tag}}", tag)
        .replace("{{date}}", date)
        .replace("{{sep}}", sep)
        .replace("{{branch}}", branch)
}

fn decorate_chapters<F>(item: &mut BookItem, decorate: &F)
where
    F: Fn(&mut mdbook::book::Chapter),
{
    if let BookItem::Chapter(ch) = item {
        decorate(ch);
        for sub in &mut ch.sub_items {
            decorate_chapters(sub, decorate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_basic() {
        let output = render_template(
            "Commit: {{hash}} on {{date}}",
            "abcd123",  // hash
            "",         // long_hash
            "",         // tag
            "2025-06-24",
            "|",
            "",         // branch
        );
        assert_eq!(output, "Commit: abcd123 on 2025-06-24");
    }
}
