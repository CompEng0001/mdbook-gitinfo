use crate::chapters::decorate_chapters;
use crate::config::{load_config, AlignSetting, GitInfoConfig};
use crate::git;
use crate::layout::{resolve_align, resolve_margins, resolve_messages};
use crate::repo::{resolve_repo_base, tag_url};
use crate::timefmt::format_commit_datetime;
use crate::renderer::{render_contributors_html, render_template, style_block, wrap_block};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::path::PathBuf;

pub struct GitInfo;

impl GitInfo {
    pub fn new() -> Self { GitInfo }
}

impl Preprocessor for GitInfo {
    fn name(&self) -> &str { "gitinfo" }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let cfg = load_config(ctx).unwrap_or_default();
        if !cfg.enable.unwrap_or(true) { return Ok(book); }

        let contributors_enabled = cfg.contributors.unwrap_or(false);
        const CONTRIBUTORS_TOKEN: &str = "{% contributors %}";
        let contributor_title = cfg
            .contributor_title
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("Contributors");

        let excluded_contributors: std::collections::BTreeSet<String> = cfg
            .exclude_contributors.clone()
            .unwrap_or_default()
            .into_iter()
            .collect();


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
        let repo_base = if hyperlink { resolve_repo_base(&ctx.root) } else { None };
        let resolved_tag = if let Some(t) = cfg.tag.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            t.to_string()
        } else {
            git::latest_tag_for_branch(&branch, &ctx.root)
        };

        if !git::verify_branch(&branch, &ctx.root) {
            eprintln!("[mdbook-gitinfo] Warning: Branch '{}' not found, falling back to 'main'", branch);
            branch = "main".to_string();
        }

        let contributors_html: Option<String> = if contributors_enabled {
            match git::get_contributor_usernames_from_shortlog(&ctx.root) {
                Ok(users) => {
                    let filtered: Vec<String> = users
                        .into_iter()
                        .filter(|u| !excluded_contributors.contains(u))
                        .collect();

                    Some(render_contributors_html(&filtered, contributor_title))
                }
                Err(e) => {
                    eprintln!("[mdbook-gitinfo] Warning: unable to get contributors: {e}");
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

                    // Replace token with contributors block (or remove token if disabled).
                    if ch.content.contains(CONTRIBUTORS_TOKEN) {
                        if let Some(html) = contributors_html.as_ref() {
                            ch.content = ch.content.replace(CONTRIBUTORS_TOKEN, html);
                        } else {
                            ch.content = ch.content.replace(CONTRIBUTORS_TOKEN, "");
                        }
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

    fn supports_renderer(&self, renderer: &str) -> bool { renderer == "html" }
}
