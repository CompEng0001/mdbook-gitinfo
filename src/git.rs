//! Utility module for running Git commands.
//!
//! This module provides helpers for interacting with a Git repository,
//! primarily to extract metadata (commit hash, tag, timestamp, branch).
//!
//! All functions return [`mdbook::errors::Error`] on failure so they can be
//! integrated directly into the `mdbook` preprocessor error flow.
//!
//! See also:
//! - [`get_git_output`] — Run arbitrary Git commands and capture output.
//! - [`verify_branch`] — Convenience wrapper to check branch existence.

use mdbook_preprocessor::errors::Error;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

/// Run a Git command and return the trimmed `stdout` output as a [`String`].
///
/// This is the central utility for invoking Git. It is used by the
/// `mdbook-gitinfo` preprocessor to fetch commit information such as:
/// - short or long commit hash
/// - nearest tag
/// - commit date/time
///
/// See also: [`verify_branch`], which builds on this function to check
/// if a branch exists locally.
///
/// # Type Parameters
///
/// - `I`: An iterator of arguments (e.g., a string slice array).
/// - `S`: Each argument, convertible to [`OsStr`].
///
/// # Arguments
///
/// * `args` — Git command-line arguments (e.g., `["rev-parse", "HEAD"]`).
/// * `dir` — Path to the Git repository root or working directory.
///
/// # Returns
///
/// * `Ok(String)` — Trimmed `stdout` output from Git.
/// * `Err(Error)` — If Git fails to launch or exits with non-zero status.
///
/// # Errors
///
/// This function returns an [`Error`] if:
/// - The `git` binary is missing or fails to start.
/// - The command returns a non-zero exit code.
/// - The output cannot be decoded as UTF-8.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use mdbook_gitinfo::git::get_git_output;
///
/// let hash = get_git_output(["rev-parse", "--short", "HEAD"], Path::new("."))
///     .expect("failed to get commit hash");
/// println!("Current short commit hash: {}", hash);
/// ```
pub fn get_git_output<I, S>(args: I, dir: &Path) -> Result<String, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| Error::msg(format!("Git command failed: {e}")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(Error::msg("Git command returned non-zero exit code"))
    }
}

/// Verify that a branch exists locally in the given repository.
///
/// Internally runs:
/// ```text
/// git rev-parse --verify <branch>
/// ```
///
/// This is a thin wrapper around [`get_git_output`], returning `true` if the
/// Git call succeeds and `false` otherwise.
///
/// # Arguments
///
/// * `branch` — The name of the branch to check.
/// * `dir` — Path to the Git repository root or working directory.
///
/// # Returns
///
/// * `true` if the branch exists locally.
/// * `false` otherwise.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use mdbook_gitinfo::git::verify_branch;
///
/// let dir = Path::new(".");
/// if !verify_branch("dev", dir) {
///     eprintln!("Branch 'dev' not found, falling back to 'main'");
/// }
/// ```
pub fn verify_branch(branch: &str, dir: &Path) -> bool {
    get_git_output(["rev-parse", "--verify", branch], dir).is_ok()
}

/// Return the latest tag name, preferring tags reachable from the given branch's HEAD.
/// Falls back to global (by creator date) when describe fails.
/// Returns "No tags found" if not tag found
pub fn latest_tag_for_branch(branch: &str, dir: &std::path::Path) -> String {
    // Prefer a tag reachable from branch HEAD
    if let Ok(t) = get_git_output(["describe", "--tags", "--abbrev=0", branch], dir) {
        if !t.trim().is_empty() {
            return t;
        }
    }

    // Fallback: newest tag by creator date
    match get_git_output(["tag", "--sort=-creatordate"], dir) {
        Ok(list) => {
            if let Some(first) = list.lines().find(|l| !l.trim().is_empty()) {
                return first.trim().to_string();
            }
        }
        Err(_) => {}
    }

    "No tags found".to_string()
}

/// Extract a GitHub username from a GitHub noreply email address.
///
/// Supported patterns:
/// - `username@users.noreply.github.com`
/// - `12345+username@users.noreply.github.com`
fn github_username_from_email(email: &str) -> Option<String> {
    const SUFFIX: &str = "@users.noreply.github.com";
    if !email.ends_with(SUFFIX) {
        return None;
    }
    let local = &email[..email.len() - SUFFIX.len()];
    let local = local.trim();
    if local.is_empty() {
        return None;
    }
    // Strip optional numeric prefix: "12345+username"
    let username = match local.split_once('+') {
        Some((_id, u)) if !u.trim().is_empty() => u.trim(),
        _ => local,
    };
    if username.is_empty() {
        None
    } else {
        Some(username.to_string())
    }
}

fn is_plausible_github_username(u: &str) -> bool {
    // Conservative subset: 1–39 chars of [A-Za-z0-9-], not starting/ending with '-'
    let len = u.len();
    if len == 0 || len > 39 {
        return false;
    }
    if u.starts_with('-') || u.ends_with('-') {
        return false;
    }
    u.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Retrieve contributor usernames from `git shortlog -sne --all`.
///
/// Strategy:
/// 1) Prefer the *author name* if it looks like a GitHub username.
/// 2) Otherwise, fallback to extracting a username from GitHub noreply email.
///
/// Returns a unique, sorted list of inferred GitHub usernames.
pub fn get_contributor_usernames_from_shortlog(dir: &Path) -> Result<Vec<String>, Error> {
    let raw = get_git_output(["shortlog", "-sne", "--all"], dir)
        .map_err(|e| Error::msg(format!("unable to get contributors: {e}")))?;

    let mut set = BTreeSet::<String>::new();

    for line in raw.lines() {
        // Expected: "  42  Name <email>"
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split count from rest
        let mut parts = line.splitn(2, char::is_whitespace);
        let _count_str = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim();
        if rest.is_empty() {
            continue;
        }

        // Extract name and optional email
        let (name, email) = if let Some((n, e)) = rest.rsplit_once('<') {
            let email = e.trim_end_matches('>').trim();
            (n.trim(), Some(email))
        } else {
            (rest, None)
        };

        // 1) Prefer author name (if plausible)
        if !name.is_empty() && is_plausible_github_username(name) {
            set.insert(name.to_string());
            continue;
        }

        // 2) Fallback to email-derived username (GitHub noreply only)
        if let Some(email) = email {
            if let Some(u) = github_username_from_email(email) {
                if is_plausible_github_username(&u) {
                    set.insert(u);
                }
            }
        }
    }

    Ok(set.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn returns_error_on_invalid_git_command() {
        let result = get_git_output(["non-existent-command"], &PathBuf::from("."));
        assert!(result.is_err());
    }
}
