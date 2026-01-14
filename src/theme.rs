use mdbook_preprocessor::PreprocessorContext;
use std::fs;
use std::io;
use toml_edit::{DocumentMut, Item, Value};

const CSS_REL_PATH: &str = "theme/gitinfo.css";

pub fn ensure_gitinfo_assets(ctx: &PreprocessorContext, css_contents: &str) {
    if let Err(e) = ensure_css_file(ctx, css_contents) {
        eprintln!("[mdbook-gitinfo] Warning: unable to write {}: {e}", CSS_REL_PATH);
    }

    if let Err(e) = ensure_book_toml_additional_css(ctx) {
        eprintln!("[mdbook-gitinfo] Warning: unable to update book.toml additional-css: {e}");
    }
}

fn ensure_css_file(ctx: &PreprocessorContext, css_contents: &str) -> io::Result<()> {
    // Put CSS under the mdBook theme override directory at the repo root.
    // This avoids needing to modify the book source directory layout.
    let theme_dir = ctx.root.join("theme");
    fs::create_dir_all(&theme_dir)?;

    let css_path = theme_dir.join("gitinfo.css");

    // Idempotent write: only write if missing or different.
    match fs::read_to_string(&css_path) {
        Ok(existing) if existing == css_contents => Ok(()),
        _ => fs::write(&css_path, css_contents),
    }
}


fn ensure_book_toml_additional_css(ctx: &PreprocessorContext) -> io::Result<()> {
    let book_toml = ctx.root.join("book.toml");

    // If book.toml doesn't exist (rare), do nothing gracefully.
    if !book_toml.exists() {
        return Ok(());
    }

    let raw = fs::read_to_string(&book_toml)?;
    let mut doc: DocumentMut = raw.parse().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("invalid book.toml: {e:?}"))
    })?;

    // Ensure [output.html] table exists
    if doc.get("output").is_none() {
        doc["output"] = toml_edit::table().into();
    }
    if doc["output"].get("html").is_none() {
        doc["output"]["html"] = toml_edit::table().into();
    }

    // Ensure output.html.additional-css is an array, then append if missing.
    let item = doc["output"]["html"].get_mut("additional-css");

    match item {
        None | Some(Item::None) => {
            let mut arr = toml_edit::Array::default();
            arr.push(Value::from(CSS_REL_PATH));
            doc["output"]["html"]["additional-css"] = Item::Value(Value::Array(arr));
        }

        Some(Item::Value(Value::Array(arr))) => {
            let already = arr.iter().any(|v| v.as_str() == Some(CSS_REL_PATH));
            if !already {
                arr.push(Value::from(CSS_REL_PATH));
            }
        }

        // Sometimes users set a single string instead of an array; normalize to array.
        Some(Item::Value(Value::String(s))) => {
            let existing = s.value().to_string();
            let needs_css = existing != CSS_REL_PATH;

            let mut arr = toml_edit::Array::default();
            arr.push(Value::from(existing));
            if needs_css {
                arr.push(Value::from(CSS_REL_PATH));
            }

            doc["output"]["html"]["additional-css"] = Item::Value(Value::Array(arr));
        }

        Some(other) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "output.html.additional-css exists but is not a string or array (found: {:?})",
                    other.type_name()
                ),
            ));
        }
    }

    let updated = doc.to_string();

    // Idempotent write: only write if it changed
    if updated != raw {
        fs::write(&book_toml, updated)?;
    }

    Ok(())
}
