# Welcome to mdbook-qitinfo

This a <a href="https://github.com/rust-lang/mdBook">mdBook</a> preprocessor that injects Git metadata (commit hash, full hash, tag, date/time, branch) into each chapter — as a header, a footer, or both — with flexible templates, alignment, and CSS-style margins.

For all options see [Documentation](./Documentation.md) chapter.

## Live Configuration Example 

As seen from this page the current preprocessor configuration is: 

```toml
[preprocessor.gitinfo]
enable = true
header = true
footer = true
message.header = "Last updated: <em>{{date}}</em>"
message.footer = "tag: {{tag}} {{sep}} branch: {{branch}} {{sep}} commit: {{hash}}"
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
