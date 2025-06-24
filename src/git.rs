//! Utility module for running Git commands.
//!
//! This module defines a helper function to invoke Git from a given directory
//! and capture trimmed output, returning a well-formed `mdbook::Error` on failure.

use mdbook::errors::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

/// Run a Git command and return the trimmed `stdout` output as a `String`.
///
/// This function is used by the preprocessor to extract Git metadata such as commit hashes,
/// tags, and commit timestamps.
///
/// # Type Parameters
///
/// - `I`: An iterator over arguments implementing `AsRef<OsStr>`, typically a string slice array.
/// - `S`: An individual item type in the iterator, convertible to `OsStr`.
///
/// # Arguments
///
/// * `args` - An iterator of Git command-line arguments (e.g., `["rev-parse", "HEAD"]`).
/// * `dir` - The path to the Git repository root or working directory to execute from.
///
/// # Returns
///
/// * `Ok(String)` containing the trimmed stdout of the Git command.
/// * `Err(Error)` if the command fails to launch or returns a non-zero exit code.
///
/// # Errors
///
/// This function returns an [`mdbook::errors::Error`] if:
/// - The `git` binary is not available or fails to launch.
/// - The command exits with a non-zero status code.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use mdbook_gitinfo::git::get_git_output;
///
/// let hash = get_git_output(["rev-parse", "--short", "HEAD"], Path::new(".")).unwrap();
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
