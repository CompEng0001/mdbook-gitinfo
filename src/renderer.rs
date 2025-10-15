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
