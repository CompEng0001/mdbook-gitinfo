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
        .map_err(|e| {
            mdbook_preprocessor::errors::Error::msg(format!("contributors template error: {e}"))
        })?;

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

    hb.render("contributors", &ctx).map_err(|e| {
        mdbook_preprocessor::errors::Error::msg(format!("contributors render error: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_replaces_all_placeholders() {
        let t = "{{date}}{{sep}}commit: {{hash}} ({{long}}) tag={{tag}} branch={{branch}}";
        let out = render_template(
            t,
            "abc123",
            "abc123def456",
            "v1.2.3",
            "2026-01-14 12:34:56",
            " • ",
            "main",
        );

        assert!(out.contains("2026-01-14 12:34:56"));
        assert!(out.contains(" • "));
        assert!(out.contains("commit: abc123"));
        assert!(out.contains("(abc123def456)"));
        assert!(out.contains("tag=v1.2.3"));
        assert!(out.contains("branch=main"));
        assert!(!out.contains("{{hash}}"));
        assert!(!out.contains("{{long}}"));
        assert!(!out.contains("{{tag}}"));
        assert!(!out.contains("{{date}}"));
        assert!(!out.contains("{{sep}}"));
        assert!(!out.contains("{{branch}}"));
    }

    #[test]
    fn render_template_leaves_unknown_placeholders_untouched() {
        let t = "x={{hash}} y={{unknown}}";
        let out = render_template(t, "h", "lh", "t", "d", "|", "b");
        assert_eq!(out, "x=h y={{unknown}}");
    }

    #[test]
    fn style_block_formats_expected_css() {
        let margin = [
            "1em".to_string(),
            "2em".to_string(),
            "3em".to_string(),
            "4em".to_string(),
        ];

        let out = style_block("0.8em", "center", &margin);

        assert_eq!(
            out,
            "font-size:0.8em;padding:4px;margin:1em 2em 3em 4em;text-align:center;display:block;"
        );
    }

    #[test]
    fn wrap_block_header() {
        let out = wrap_block(true, "font-size:1em;", "hello");
        assert_eq!(
            out,
            r#"<header class="gitinfo-header" style="font-size:1em;">hello</header>"#
        );
    }

    #[test]
    fn wrap_block_footer() {
        let out = wrap_block(false, "font-size:1em;", "hello");
        assert_eq!(
            out,
            r#"<footer class="gitinfo-footer" style="font-size:1em;">hello</footer>"#
        );
    }

    #[test]
    fn gitinfo_css_is_present_and_has_expected_selector() {
        // Guard: accidental empty file / wrong include path.
        assert!(!GITINFO_CSS.trim().is_empty());
        // Guard: accidental regression where the key class disappears.
        assert!(GITINFO_CSS.contains(".contributor-footnotes"));
    }

    #[test]
    fn render_contributors_hbs_renders_visible_and_hidden_users_and_title() {
        let visible = vec!["author1".to_string(), "author2".to_string()];
        let hidden = vec!["author3".to_string()];

        let html = render_contributors_hbs("Contributors", None, &visible, &hidden)
            .expect("contributors template should render");

        // Title
        assert!(html.contains("Contributors"));

        // Visible users should appear somewhere in the HTML
        assert!(html.contains("author1"));
        assert!(html.contains("author2"));

        // Hidden users should appear if provided
        assert!(html.contains("author3"));
    }

    #[test]
    fn render_contributors_hbs_includes_raw_html_message_when_provided() {
        let visible = vec!["author1".to_string()];
        let hidden: Vec<String> = vec![];

        let msg = Some("<em>Thanks!</em>");
        let html = render_contributors_hbs("Contributors", msg, &visible, &hidden)
            .expect("contributors template should render");

        // This test assumes the template uses triple-stash {{{message}}}
        // so that raw HTML is not escaped.
        assert!(html.contains("<em>Thanks!</em>"));
        assert!(!html.contains("&lt;em&gt;Thanks!&lt;/em&gt;"));
    }
}
