use handlebars::{Handlebars,};
use serde::Serialize;

const CONTRIBUTORS_TEMPLATE: &str = include_str!("../templates/contributor.hbs");

#[derive(Serialize)]
struct ContributorsCtx<'a> {
    title: &'a str,
    // Render raw HTML from config (trusted).
    message: Option<String>,
    usernames: &'a [String],
}


/// Render string template with placeholders.
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

/// Build inline CSS style string.
pub fn style_block(font_size: &str, align: &str, margin: &[String;4]) -> String {
    fn css_margin_string(margin: &[String; 4]) -> String {
        format!("{} {} {} {}", margin[0], margin[1], margin[2], margin[3])
    }
    format!(
        "font-size:{};padding:4px;margin:{};text-align:{};display:block;",
        font_size, css_margin_string(margin), align
    )
}

/// Wrap HTML into header/footer element.
pub fn wrap_block(is_header: bool, style: &str, html: &str) -> String {
    if is_header {
        format!(r#"<header class="gitinfo-header" style="{}">{}</header>"#, style, html)
    } else {
        format!(r#"<footer class="gitinfo-footer" style="{}">{}</footer>"#, style, html)
    }
}

pub fn render_contributors_hbs(
    title: &str,
    contributor_message: Option<&str>,
    usernames: &[String],
) -> Result<String, mdbook::errors::Error> {
    let mut hb = Handlebars::new();
    hb.register_template_string("contributors", CONTRIBUTORS_TEMPLATE)
        .map_err(|e| mdbook::errors::Error::msg(format!("contributors template error: {e}")))?;

    let ctx = ContributorsCtx {
        title,
        message: contributor_message
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| String::from(s.to_string())),
        usernames,
    };

    hb.render("contributors", &ctx)
        .map_err(|e| mdbook::errors::Error::msg(format!("contributors render error: {e}")))
}
