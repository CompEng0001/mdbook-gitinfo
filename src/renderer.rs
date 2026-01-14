use handlebars::Handlebars;
use serde::Serialize;

const CONTRIBUTORS_TEMPLATE: &str = include_str!("../templates/contributor.hbs");
pub const GITINFO_CSS: &str = include_str!("../templates/gitinfo.css");

#[derive(Serialize)]
struct ContributorsCtx<'a> {
    title: &'a str,
    // Render raw HTML from config (trusted).
    message: Option<String>,
    usernames_visible: &'a [String],
    usernames_hidden: &'a [String],
    hidden_count: usize,
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
pub fn style_block(font_size: &str, align: &str, margin: &[String; 4]) -> String {
    fn css_margin_string(margin: &[String; 4]) -> String {
        format!("{} {} {} {}", margin[0], margin[1], margin[2], margin[3])
    }
    format!(
        "font-size:{};padding:4px;margin:{};text-align:{};display:block;",
        font_size,
        css_margin_string(margin),
        align
    )
}

/// Wrap HTML into header/footer element.
pub fn wrap_block(is_header: bool, style: &str, html: &str) -> String {
    if is_header {
        format!(
            r#"<header class="gitinfo-header" style="{}">{}</header>"#,
            style, html
        )
    } else {
        format!(
            r#"<footer class="gitinfo-footer" style="{}">{}</footer>"#,
            style, html
        )
    }
}

pub fn render_contributors_hbs(
    title: &str,
    contributors_message: Option<&str>,
    usernames_visible: &[String],
    usernames_hidden: &[String],
) -> Result<String, mdbook_preprocessor::errors::Error> {
    let mut hb = Handlebars::new();
    hb.register_template_string("contributors", CONTRIBUTORS_TEMPLATE)
        .map_err(|e| mdbook_preprocessor::errors::Error::msg(format!("contributors template error: {e}")))?;

    let hidden_count = usernames_hidden.len();
    let ctx = ContributorsCtx {
        title,
        message: contributors_message
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| String::from(s.to_string())),
        usernames_visible,
        usernames_hidden,
        hidden_count,
    };

    hb.render("contributors", &ctx)
        .map_err(|e| mdbook_preprocessor::errors::Error::msg(format!("contributors render error: {e}")))
}
