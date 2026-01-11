use crate::git;

fn normalise_repo_base(url: &str) -> String {
    let u = url.trim().trim_end_matches(".git").to_string();
    if let Some(rest) = u.strip_prefix("git@github.com:") {
        return format!("https://github.com/{rest}");
    }
    if let Some(rest) = u.strip_prefix("ssh://git@github.com/") {
        return format!("https://github.com/{rest}");
    }
    u
}

/// Detect repository base URL across CI providers, or from local git remote.
pub fn resolve_repo_base(ctx_root: &std::path::Path) -> Option<String> {
    if let (Ok(server), Ok(repo)) = (
        std::env::var("GITHUB_SERVER_URL"),
        std::env::var("GITHUB_REPOSITORY"),
    ) {
        return Some(format!("{}/{}", server.trim_end_matches('/'), repo));
    }
    if let (Ok(server), Ok(path)) = (
        std::env::var("CI_SERVER_URL"),
        std::env::var("CI_PROJECT_PATH"),
    ) {
        return Some(format!("{}/{}", server.trim_end_matches('/'), path));
    }
    if let Ok(http_origin) = std::env::var("BITBUCKET_GIT_HTTP_ORIGIN") {
        return Some(normalise_repo_base(&http_origin));
    }
    if let Ok(full) = std::env::var("BITBUCKET_REPO_FULL_NAME") {
        return Some(format!("https://bitbucket.org/{}", full));
    }
    if let Ok(remote) = git::get_git_output(["config", "--get", "remote.origin.url"], ctx_root) {
        return Some(normalise_repo_base(&remote));
    }
    None
}

pub fn tag_url(base: &str, tag: &str) -> String {
    if base.contains("github.com") {
        format!("{}/releases/tag/{}", base, tag)
    } else if base.contains("gitlab") {
        format!("{}/-/tags/{}", base, tag)
    } else if base.contains("bitbucket.org") {
        format!("{}/src/{}", base, tag)
    } else {
        // generic-ish fallback
        format!("{}/tags/{}", base, tag)
    }
}
