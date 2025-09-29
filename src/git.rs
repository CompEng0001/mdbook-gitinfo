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

use mdbook::errors::Error;
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
