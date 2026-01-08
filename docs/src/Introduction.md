# Welcome to mdbook-qitinfo

<p align="center">
  <a href="https://crates.io/crates/mdbook-gitinfo">
    <img src="https://img.shields.io/crates/v/mdbook-gitinfo?style=for-the-badge" alt="Crates.io version" />
  </a>
  <a href="https://crates.io/crates/mdbook-gitinfo">
    <img src="https://img.shields.io/crates/d/mdbook-gitinfo?style=for-the-badge" alt="Downloads" />
  </a>
  <a href="https://docs.rs/mdbook-gitinfo">
    <img src="https://img.shields.io/docsrs/mdbook-gitinfo?style=for-the-badge" alt="Docs.rs" />
  </a>
  <a href="https://github.com/CompEng0001/mdbook-gitinfo/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/CompEng0001/mdbook-gitinfo/release.yml?&style=for-the-badge&label=CI" alt="CI status" />
  </a>
</p>

An <a href="https://github.com/rust-lang/mdBook">mdBook</a> preprocessor that injects Git metadata (commit hash, full hash, tag, date/time, branch) into each chapter — as a header, a footer, or both — with flexible templates, alignment, and CSS-style margins.

> [!WARNING]
> Due to break in changes currently mdbook-gitinfo works with mdbook v0.4.52, **not** 0.5.0.
> - [https://crates.io/crates/mdbook/0.4.52](https://crates.io/crates/mdbook/0.4.52)
> - [https://github.com/rust-lang/mdBook/releases/tag/v0.4.52](https://github.com/rust-lang/mdBook/releases/tag/v0.4.52)

<br>

For all options see [Documentation](./Documentation.md) chapter.

## Live Configuration Example 

As seen from this page the current preprocessor configuration is: 

```toml
[preprocessor.gitinfo]
enable = true
header = true
footer = true
message.header = "Last updated: <em>{{date}}</em>"
message.footer = "branch: {{branch}} {{sep}} commit: {{hash}} {{sep}} tag: {{tag}}"
align.header = "center"
align.footer = "center"
margin.header.top = "2em"
margin.header.bottom = "2em"
margin.footer = ["2em", "0", "0", "0"]
font-size = "0.8em"
separator = "||"
date-format = "%A %d %B %Y"
time-format = "@ %H:%M:%S"
branch = "main"
hyperlink = true
```

{% contributors %}