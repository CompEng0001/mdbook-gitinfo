//! mdbook-gitinfo — mdBook preprocessor that injects Git metadata.
//!
//! See [`config`] for user configuration and [`git`] for Git helpers.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod config;
pub mod git;
pub mod theme;

#[doc(inline)]
pub use config::GitInfoConfig;

#[doc(inline)]
pub use git::{get_git_output, verify_branch};

#[doc(inline)]
pub use theme::ensure_gitinfo_assets;
