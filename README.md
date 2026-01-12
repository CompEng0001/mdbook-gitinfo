<div align="center">
  <h1 align="center"><b>mdbook-gitinfo</b></h1>
</div>

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
  <img src="https://img.shields.io/badge/Built%20with-Rust-orange?logo=rust&style=for-the-badge" alt="Built with Rust" />
</p>

An <a href="https://github.com/rust-lang/mdBook">mdBook</a> preprocessor that injects Git metadata (commit hash, full hash, tag, date/time, branch) into each chapter — as a header, a footer, or both — with flexible templates, alignment, and CSS-style margins.

---

## Features

- Injects the latest Git commit information **per chapter** (and subchapters).
- **Header and/or footer** placement with independent configuration.
- Message templating via `message.header` / `message.footer` / `message.both`.
- Split **alignment** for header/footer or a single legacy value.
- **CSS-style margins (TRBL)** with per-placement overrides and shorthand.
- **Timezone-aware date/time rendering** (local, UTC, source, or fixed offset).
- Optional **hyperlinks** for commit and branch to your remote provider.
- Branch verification with graceful fallback to `"main"`.
- Only the `html` renderer is supported.
- Display list of contributors via git, file, and inline methods

---

## Installation

From crates.io:

```sh
cargo install mdbook-gitinfo
```

From source (in this repo):

```sh
cargo install --path .
```

Ensure the `mdbook-gitinfo` binary is on your `PATH`.

---

## Quick start

Add to `book.toml`:

```toml
[preprocessor.gitinfo]
enable = true

# choose placement(s)
header = false   # default: false
footer = true    # default: true

# common formatting
font-size   = "0.9em"
separator   = " • "
date-format = "%Y-%m-%d"
time-format = "%H:%M:%S"
branch      = "main"
hyperlink   = true  # make hash/branch clickable when possible

# optional timezone handling
timezone = "local"   # "local" (default) | "utc" | "source" | "fixed:+02:00"

[preprocessor.gitinfo.message]
footer = "Built {{date}}{{sep}}commit: {{hash}}"
```

---

## Configuration via **dotted keys** (with table equivalents)

You can configure options either with **dotted keys** under `[preprocessor.gitinfo]` or with nested **tables** like `[preprocessor.gitinfo.message]`. Use **one style consistently** for readability; both work and merge as expected.

### Timezone

The new timezone option controls how commit timestamps are rendered.

| Value                            | Description                                              |
| -------------------------------- | -------------------------------------------------------- |
| `local` *(default)*              | Convert to system local time.                            |
| `utc`                            | Convert to Coordinated Universal Time (UTC).             |
| `source`                         | Use the commit’s recorded timezone offset (as authored). |
| `fixed:+HH:MM` or `fixed:-HH:MM` | Force a specific fixed offset.                           |
| *anything else*                  | Emits a warning and falls back to `local`.               |

>[!NOTE]
>The offset is always applied, but not shown unless you include `%z`, `%:z`, or `%Z` in your time-format

### Message templates

**Placeholders:** `{{hash}}`, `{{long}}`, `{{tag}}`, `{{date}}`, `{{sep}}`, `{{branch}}`  
**Precedence (per placement):** `message.header/footer` ➝ `message.both` ➝ legacy `header_message/footer_message` ➝ legacy `template`.

> If a placement-specific template is set (`message.header` or `message.footer`), `message.both` is ignored <em>for that placement</em>.

**Dotted keys:**
```toml
[preprocessor.gitinfo]
message.header = "Last updated: <em>{{date}}</em>"
message.footer = "branch: {{branch}}{{sep}}commit: {{hash}}"
message.both   = "<em>{{date}}</em>{{sep}}branch: {{branch}}"
```

**Table form (equivalent):**
```toml
[preprocessor.gitinfo.message]
header = "Last updated: <em>{{date}}</em>"
footer = "branch: {{branch}}{{sep}}commit: {{hash}}"
both   = "<em>{{date}}</em>{{sep}}branch: {{branch}}"
```

---

### Align

Values: `"left" | "center" | "right"` (default **center** for both).

**Resolution:** `align.header` and/or `align.footer` override `align.both`.  
If neither is set, both default to `"center"`.

**Dotted keys:**
```toml
[preprocessor.gitinfo]
align.header = "left"
align.footer = "right"
align.both   = "center"   # used only for any placement not explicitly set
```

**Table form (equivalent):**
```toml
[preprocessor.gitinfo.align]
both   = "center"
header = "left"
footer = "right"
```

---

### Margin (TRBL)

Margins accept CSS-style **T**op **R**ight **B**ottom **L**eft values. Units can be `px`, `em`, etc., or unitless (`0`).  
You can provide:
- a **single value** (applies to all sides),
- an **array** with 1–4 items (CSS shorthand),
- or **named sides** (`top/right/bottom/left`).

**Resolution:** `margin.header` / `margin.footer` override `margin.both` per placement.

**Defaults (when unset):**
- **Header:** `["0", "0", "2em", "0"]` (space **below** the header block)
- **Footer:** `["2em", "0", "0", "0"]` (space **above** the footer block)
- Legacy `margin-top` (if present) is treated as **footer top** spacing.

**Dotted keys (array shorthand):**
```toml
[preprocessor.gitinfo]
margin.header = ["0", "0", "1.25em", "0"]  # T R B L
margin.footer = ["2em", "0", "0", "0"]     # T R B L
margin.both   = "1em"                      # all sides = 1em unless overridden
```

**Dotted keys (named sides):**
```toml
[preprocessor.gitinfo]
margin.header.top     = "5em"
margin.footer.bottom  = "2em"
margin.both.left      = "0.5em"
```

**Table form (equivalent):**
```toml
[preprocessor.gitinfo.margin]
both   = ["1em"]                     # all sides = 1em
header = ["0", "0", "1.25em", "0"]   # T R B L
footer = { top = "2em" }             # named sides form
```

---

## Placeholders

Use these tokens inside your message templates:

- `{{hash}}` — short commit hash
- `{{long}}` — full commit hash
- `{{tag}}` — nearest tag
- `{{date}}` — commit date and time (combined using your `date-format` and `time-format`)
- `{{sep}}` — the configured separator (e.g., `" • "`)
- `{{branch}}` — branch name

---

## Example output

With the configuration above, a footer will be injected similar to:

```html
<footer class="gitinfo-footer" style="font-size:0.8em;padding:4px;margin:2em 0 0 0;text-align:center;display:block;">
  branch: <b><a href="somelinktosomeawesomerepo">main</a></b> • commit: <a href="somelinktosomeawesomerepo">9296b47</a>
</footer>

```

> The preprocessor inserts blank lines around injected blocks so Markdown headings/paragraphs render correctly.

---

## Formatting & Git options

- `font-size` — e.g., `"0.8em"`
- `separator` — string used by `{{sep}}`
- `date-format`, `time-format` — chrono formatting strings (examples below)
- `branch` — default `"main"`. If the branch isn’t found, the preprocessor falls back to `"main"` with a warning.
- `hyperlink` — when `true`, `{{hash}}` and `{{branch}}` are linked to your provider (derived from CI env vars like `GITHUB_SERVER_URL`/`GITHUB_REPOSITORY`, `CI_SERVER_URL`/`CI_PROJECT_PATH`, Bitbucket vars, or `remote.origin.url`).

### Common chrono format specifiers

For DateTime format specifiers refer to `chrono`::`format`:
- [https://docs.rs/chrono/latest/chrono/format/strftime/index.html](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

---

## CI note (GitHub Actions)

When using `actions/checkout@v4`, set `fetch-depth: 0` so the plugin can access commit history:

```yml
jobs:
  deploy:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      # … your build & deploy steps
```

---

## Compatibility

- Tested with **mdBook 0.4.x**.
- Renderer support: **html** only.

---

## License

[Apache-2.0](LICENSE.md)

---

## Author

[CompEng0001](https://github.com/CompEng0001)
