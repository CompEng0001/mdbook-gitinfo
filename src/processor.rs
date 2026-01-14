use crate::chapters::decorate_chapters;
use crate::config::{load_config, ContributorsSource};
use crate::git;
use crate::layout::{resolve_align, resolve_margins, resolve_messages};
use crate::renderer::{GITINFO_CSS, render_contributors_hbs, render_template, style_block, wrap_block};
use crate::repo::{resolve_repo_base, tag_url};
use crate::theme::ensure_gitinfo_assets;
use crate::timefmt::format_commit_datetime;
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use std::{fs, path::PathBuf};

pub struct GitInfo;

impl GitInfo {
    pub fn new() -> Self {
        GitInfo
    }
}

/// Extract all `{% contributors ... %}` tokens and replace them with rendered HTML.
/// Ignores fence blocks
///
/// Token forms:
/// - `{% contributors %}`
/// - `{% contributors a b c %}` (only honoured when contributors-source = "inline")
fn replace_contributors_tokens(
    input: &str,
    source: ContributorsSource,
    contributors_html_global: &str,
    inline_renderer: &dyn Fn(&[String]) -> String,
) -> String {
    let mut out = String::with_capacity(input.len());

    // fenced code tracking
    let mut in_fence = false;
    let mut fence_ch: char = '\0'; // '`' or '~'
    let mut fence_len: usize = 0;

    for line in input.split_inclusive('\n') {
        // Detect fenced blocks (``` or ~~~), allowing leading spaces/tabs
        let trimmed = line.trim_start_matches(|c| c == ' ' || c == '\t');

        if !trimmed.is_empty() {
            let first = trimmed.chars().next().unwrap();
            if first == '`' || first == '~' {
                let mut count = 0usize;
                for c in trimmed.chars() {
                    if c == first {
                        count += 1;
                    } else {
                        break;
                    }
                }
                if count >= 3 {
                    if !in_fence {
                        in_fence = true;
                        fence_ch = first;
                        fence_len = count;
                        out.push_str(line);
                        continue;
                    } else if first == fence_ch && count >= fence_len {
                        in_fence = false;
                        fence_ch = '\0';
                        fence_len = 0;
                        out.push_str(line);
                        continue;
                    }
                }
            }
        }

        // If inside a fenced code block, do not evaluate tokens.
        if in_fence {
            out.push_str(line);
            continue;
        }

        // Skip indented code blocks (CommonMark): lines beginning with 4 spaces or a tab.
        if line.starts_with('\t') || line.starts_with("    ") {
            out.push_str(line);
            continue;
        }

        // Only replace when the token is the entire (trimmed) line.
        // This prevents replacement inside tables, inline code, or prose.
        let t = line.trim();
        if t.starts_with("{%") && t.ends_with("%}") {
            let inner = t.trim_start_matches("{%").trim_end_matches("%}").trim();
            if inner.starts_with("contributors") {
                let mut parts = inner.split_whitespace();
                let _kw = parts.next();
                let args: Vec<String> = parts
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                let html = match source {
                    ContributorsSource::Inline => {
                        if args.is_empty() {
                            eprintln!("[mdbook-gitinfo] Warning: contributors-source is 'inline' but no usernames provided in '{{% contributors %}}'");
                            String::new()
                        } else {
                            inline_renderer(&args)
                        }
                    }
                    ContributorsSource::Git | ContributorsSource::File => {
                        if !args.is_empty() {
                            eprintln!("[mdbook-gitinfo] Warning: inline contributors list ignored because contributors-source is not 'inline'");
                        }
                        contributors_html_global.to_string()
                    }
                };

                // Emit as a raw HTML block with blank lines around it
                out.push_str("\n");
                out.push_str(html.trim());
                out.push_str("\n\n");
                continue;
            }
        }

        // Default: unchanged
        out.push_str(line);
    }

    out
}


fn parse_contributors_file(path: &std::path::Path) -> Vec<String> {
    let Ok(raw) = fs::read_to_string(path) else {
        return vec![];
    };
    raw.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.strip_prefix("- ")
                .or_else(|| l.strip_prefix("* "))
                .unwrap_or(l)
        })
        .map(|l| l.trim().to_string())
        .collect()
}

impl Preprocessor for GitInfo {
    fn name(&self) -> &str {
        "gitinfo"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = load_config(ctx).unwrap_or_default();
        if !cfg.enable.unwrap_or(true) {
            return Ok(book);
        }

        let contributors_enabled = cfg.contributors.unwrap_or(false);
        let contributors_source = cfg.contributors_source.unwrap_or_default();
        let contributors_file = cfg
            .contributors_file
            .clone()
            .unwrap_or_else(|| "CONTRIBUTORS.md".to_string());
        // Generate assets and update book.toml once per run (no per-chapter side effects)
        if contributors_enabled {
            ensure_gitinfo_assets(ctx, GITINFO_CSS);
        }
        let contributors_title = cfg
            .contributors_title
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("Contributors");

        // Optional message (HTML) injected via {{{message}}} in the template.
        // Keep as Option so the template can {{#if message}}.
        let contributors_message: Option<&str> = cfg
            .contributors_message
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let excluded_contributors: std::collections::BTreeSet<String> = cfg
            .contributors_exclude
            .clone()
            .unwrap_or_default()
            .into_iter()
            .collect();

        let contributors_max_visible = cfg.contributors_max_visible.unwrap_or(24);

        let show_header = cfg.header.unwrap_or(false);
        let show_footer = cfg.footer.unwrap_or(true);
        let (header_tmpl, footer_tmpl) = resolve_messages(&cfg);
        let font_size = cfg.font_size.unwrap_or_else(|| "0.8em".to_string());
        let (align_header, align_footer) = resolve_align(&cfg.align);
        let (margin_header, margin_footer) = resolve_margins(&cfg.margin);
        let separator = cfg.separator.unwrap_or_else(|| " • ".to_string());
        let date_format = cfg.date_format.as_deref().unwrap_or("%Y-%m-%d");
        let time_format = cfg.time_format.as_deref().unwrap_or("%H:%M:%S");
        let mut branch = cfg.branch.unwrap_or_else(|| "main".to_string());
        let hyperlink = cfg.hyperlink.unwrap_or(false);
        let repo_base = if hyperlink {
            resolve_repo_base(&ctx.root)
        } else {
            None
        };
        let resolved_tag =
            if let Some(t) = cfg.tag.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                t.to_string()
            } else {
                git::latest_tag_for_branch(&branch, &ctx.root)
            };

        if !git::verify_branch(&branch, &ctx.root) {
            eprintln!(
                "[mdbook-gitinfo] Warning: Branch '{}' not found, falling back to 'main'",
                branch
            );
            branch = "main".to_string();
        }

        // Pre-compute the global contributors HTML for non-inline sources.
        // Inline source is resolved per token instance (args).
        let contributors_html_global: Option<String> = if contributors_enabled {
            match contributors_source {
                ContributorsSource::Git => {
                    match git::get_contributor_usernames_from_shortlog(&ctx.root) {
                        Ok(users) => {
                            let filtered: Vec<String> = users
                                .into_iter()
                                .filter(|u| !excluded_contributors.contains(u))
                                .collect();
                            let visible: Vec<String> = filtered
                                .iter()
                                .take(contributors_max_visible)
                                .cloned()
                                .collect();
                            let hidden: Vec<String> = filtered
                                .iter()
                                .skip(contributors_max_visible)
                                .cloned()
                                .collect();

                            match render_contributors_hbs(
                                contributors_title,
                                contributors_message,
                                &visible,
                                &hidden,
                            ) {
                                Ok(html) => Some(html),
                                Err(e) => {
                                    eprintln!("[mdbook-gitinfo] Warning: unable to render contributors template: {e}");
                                    Some(String::new())
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[mdbook-gitinfo] Warning: unable to get contributors from git: {e}");
                            Some(String::new())
                        }
                    }
                }
                ContributorsSource::File => {
                    let file_path = ctx.root.join(&contributors_file);
                    let users = parse_contributors_file(&file_path);
                    if users.is_empty() {
                        eprintln!("[mdbook-gitinfo] Warning: contributors-source is 'file' but no usernames found in {}", file_path.display());
                    }
                    eprintln!(
                        "[mdbook-gitinfo] contributors(file): path={} raw_lines={}",
                        file_path.display(),
                        users.len(),
                    );
                    let filtered: Vec<String> = users
                        .into_iter()
                        .filter(|u| !excluded_contributors.contains(u))
                        .collect();

                    let visible: Vec<String> = filtered
                        .iter()
                        .take(contributors_max_visible)
                        .cloned()
                        .collect();
                    let hidden: Vec<String> = filtered
                        .iter()
                        .skip(contributors_max_visible)
                        .cloned()
                        .collect();

                    eprintln!(
                        "[mdbook-gitinfo] contributors(file): path={} filtered={}",
                        file_path.display(),
                        filtered.len()
                    );
                    match render_contributors_hbs(
                        contributors_title,
                        contributors_message,
                        &visible,
                        &hidden,
                    ) {
                        Ok(html) => Some(html),
                        Err(e) => {
                            eprintln!("[mdbook-gitinfo] Warning: unable to render contributors template: {e}");
                            Some(String::new())
                        }
                    }
                }
                ContributorsSource::Inline => {
                    // Inline is per-token; global HTML is empty.
                    Some(String::new())
                }
            }
        } else {
            None
        };

        let content_dir = ctx.config.book.src.clone();

        book.for_each_mut(|item| {
            decorate_chapters(item, &|ch| {
                if let Some(path) = &ch.path {
                    let full_path = PathBuf::from(&content_dir).join(path);
                    let path_str = full_path.to_string_lossy().replace('\\', "/");

                    let short_hash = git::get_git_output(
                        ["log", "-1", "--format=%h", &branch, "--", &path_str],
                        &ctx.root,
                    ).unwrap_or_default();

                    let long_hash = git::get_git_output(
                        ["log", "-1", "--format=%H", &branch, "--", &path_str],
                        &ctx.root,
                    ).unwrap_or_default();

                    let tag = resolved_tag.clone();

                    let raw_date = git::get_git_output(
                        ["log", "-1", "--format=%cI", &branch, "--", &path_str],
                        &ctx.root,
                    ).unwrap_or_default();

                    let formatted_date = format_commit_datetime(
                        &raw_date,
                        cfg.timezone.as_deref(),
                        date_format,
                        time_format,
                    );

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

                    let tag_disp = if !tag.is_empty() && !tag.contains("No tags found") && hyperlink {
                        if let Some(base) = repo_base.as_ref() {
                            let url = tag_url(base, &tag);
                            format!(r#"<a href="{}">{}</a>"#, url, tag)
                        } else {
                            tag.clone()
                        }
                    } else {
                        "-".to_string()
                    };

                    if contributors_enabled {
                        let html_global = contributors_html_global.as_deref().unwrap_or("");

                        let inline_renderer = |args: &[String]| -> String {
                            let filtered: Vec<String> = args.iter()
                                .cloned()
                                .filter(|u| !excluded_contributors.contains(u))
                                .collect();

                            let visible: Vec<String> = filtered.iter().take(contributors_max_visible).cloned().collect();
                            let hidden: Vec<String> = filtered.iter().skip(contributors_max_visible).cloned().collect();

                            match render_contributors_hbs(contributors_title, contributors_message, &visible, &hidden) {
                                Ok(h) => h,
                                Err(e) => {
                                    eprintln!("[mdbook-gitinfo] Warning: unable to render contributors template: {e}");
                                    String::new()
                                }
                            }
                        };

                        ch.content = replace_contributors_tokens(
                            &ch.content,
                            contributors_source,
                            html_global,
                            &inline_renderer,
                        );
                    } else {
                        // If contributors disabled, strip tokens entirely.
                        ch.content = replace_contributors_tokens(
                            &ch.content,
                            contributors_source,
                            "",
                            &|_args| String::new(),
                        );
                    }

                    let render = |tmpl: &str| {
                        render_template(
                            tmpl,
                            &hash_disp,
                            &long_hash,
                            &tag_disp,
                            &formatted_date,
                            &separator,
                            &branch_disp,
                        )
                    };

                    if show_header {
                        let style = style_block(&font_size, &align_header, &margin_header);
                        let html  = wrap_block(true, &style, &render(&header_tmpl));
                        let insertion = format!("{}\n\n", html);
                        if !ch.content.starts_with(&insertion) {
                            ch.content = format!("{}{}", insertion, ch.content);
                        }
                    }

                    if show_footer {
                        let style = style_block(&font_size, &align_footer, &margin_footer);
                        let html  = wrap_block(false, &style, &render(&footer_tmpl));
                        let needs_leading_blank = !ch.content.ends_with("\n\n");
                        let prefix = if needs_leading_blank { "\n\n" } else { "\n" };
                        if !ch.content.contains(&html) {
                            ch.content.push_str(prefix);
                            ch.content.push_str(&html);
                            ch.content.push('\n');
                        }
                    }
                }
            });
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}
