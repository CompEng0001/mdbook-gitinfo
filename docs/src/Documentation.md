# Documentation

You can configure options either with dotted keys under `[preprocessor.gitinfo]` or with nested tables such as `[preprocessor.gitinfo.message]`.
Both styles merge as expected, but using one style consistently improves readability.


## 1. Core Behaviour

| Key         | Type     | Default  | Description                                                                          |
| ----------- | -------- | -------- | ------------------------------------------------------------------------------------ |
| `enable`    | `bool`   | `true`   | Master toggle for the preprocessor.                                                  |
| `header`    | `bool`   | `false`  | Render metadata at the top of each page.                                             |
| `footer`    | `bool`   | `true`   | Render metadata at the bottom of each page.                                          |
| `branch`    | `string` | `"main"` | Branch to query for commit data.                                                     |
| `hyperlink` | `bool`   | `false`  | Turns commit hash and branch into clickable links (see [Hyperlinks](#4-hyperlinks)). |
| `contributors` |`bool` | `false`  | Renders Contributors section (see [Contributors](#6-contributors)).                  |


## 2. Message Templates

Supported placeholders:
- `{{hash}}` → short commit hash
- `{{long}}` → full commit hash
- `{{tag}}`  → lastest tag or user defined - see [Tag](#32-tag)
- `{{date}}` → commit datetime - see [Date and Time](#5-date-and-time)
- `{{sep}}`  → separator string - see [Separator](#33-separator)
- `{{branch}}` → branch name as string


Precedence (per placement):

- ` message.both`  → `message.header/footer`.

> [!IMPORTANT]
> If a placement-specific template is set (`message.header` or `message.footer`), `message.both` is ignored <em>for that placement</em>.

**Example Dotted keys:**
```toml
[preprocessor.gitinfo]
message.header = "Last updated: <em>{{date}}</em>"
message.footer = "branch: {{branch}}{{sep}}commit: {{hash}}"
message.both   = "<em>{{date}}</em>{{sep}}branch: {{branch}}"
```

**Example Table form:**
```toml
[preprocessor.gitinfo.message]
header = "Last updated: <em>{{date}}</em>"
footer = "branch: {{branch}}{{sep}}commit: {{hash}}"
both   = "<em>{{date}}</em>{{sep}}branch: {{branch}}"
```


## 3. Formatting and Layout
### 3.1 Font Size

Sets the CSS font size for both header and footer text.

```toml
[preprocessor.gitinfo]
font-size = "0.9em"
```

### 3.2 Tag

Defines the git tag as a string inserted wherever `{{tag}}` appears. By default `tag` is the latest unless specified.

```toml
[preprocessor.gitinfo]
tag = "v1.1.0"
```

### 3.3 Separator

Defines the string inserted wherever `{{sep}}` appears.

```toml
[preprocessor.gitinfo]
separator = " • "
```

### 3.4 Alignment

Values: `"left"` | `"center"` | `"right"`
Default: `"center"` for both header and footer.

> [!IMPORTANT]
> If a alignment-specific template is set (`align.header` or `align.footer`), `align.both` is ignored <em>for that alignment</em>.

**Dotted keys:**
```toml
[preprocessor.gitinfo]
align.header = "left"
align.footer = "right"
align.both   = "center"
```

**Table form (equivalent):**
```toml
[preprocessor.gitinfo.align]
both   = "center"
header = "left"
footer = "right"
```

### 3.5 Margin (TRBL)

Margins accept CSS-style **T**op-**R**ight-**B**ottom-**L**eft values.

Forms accepted:

- Single value (applies to all sides)

- Array of 1–4 values (CSS shorthand)

- Object with named sides (top, right, bottom, left)

Defaults:

- Header → `["0", "0", "2em", "0"]`

- Footer → `["2em", "0", "0", "0"]`

**Example Dotted keys (array shorthand):**
```toml
[preprocessor.gitinfo]
margin.header = ["0", "0", "1.25em", "0"]
margin.footer = ["2em", "0", "0", "0"]   
margin.both   = "1em"                    
```

**Example Dotted keys (named sides):**
```toml
[preprocessor.gitinfo]
margin.header.top     = "5em"
margin.footer.bottom  = "2em"
margin.both.left      = "0.5em"
```

**Example Table form (equivalent):**
```toml
[preprocessor.gitinfo.margin]
both   = ["1em"]                  
header = ["0", "0", "1.25em", "0"]
footer = { top = "2em" }          
```

## 4. Hyperlinks

When `hyperlink = true`, the branch and commit hash become clickable links to the corresponding pages on the detected remote (GitHub).

```toml
[preprocessor.gitinfo]
hyperlink = true
message.footer = "branch {{branch}}{{sep}}commit {{hash}}"
```

> [!NOTE]
> hyperlink is constructed from the repo base name branch → commit hash 

## 5. Date and Time

Fine-tune timestamp display with `date-format`, `time-format`, `datetime_format`, and `timezone`.

| Key               | Default      | Purpose                            |
| ----------------- | ------------ | ---------------------------------- |
| `date-format`     | `"%Y-%m-%d"` | Chrono format for the date.        |
| `time-format`     | `"%H:%M:%S"` | Chrono format for the time.        |
| `datetime_format` | —            | Overrides both date and time.      |
| `show_offset`     | `false`      | Append timezone offset if missing. |
| `timezone`        | `"local"`    | See below for modes.               |

```toml
[preprocessor.gitinfo]
date-format = "%A %d %B %Y"
time-format = "@ %H:%M:%S"
show_offset = true
timezone    = "source"
```

> [!IMPORTANT]
> For DateTime format specifiers refer to `chrono`::`format`:
> - [https://docs.rs/chrono/latest/chrono/format/strftime/index.html](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

## 5.1 Timezone

Controls how commit timestamps are rendered.

| Value                           | Description                                |
| ------------------------------- | ------------------------------------------ |
| `local` *(default)*             | Convert to system local time.              |
| `utc`                           | Convert to Coordinated Universal Time.     |
| `source`                        | Use the commit’s recorded timezone offset. |
| `fixed:+HH:MM` / `fixed:-HH:MM` | Force a fixed offset.                      |
| `rfc3339`                       | Render as RFC 3339 timestamp.              |
| *anything else*                 | Emits a warning and falls back to `local`. |

<br>

> [!IMPORTANT]
> The offset is always applied, but not shown unless you include `%z`, `%:z`, or `%Z` in your time-format

## 6. Contributors

Optionally render a contributors block, showing GitHub avatars and profile links for users associated with the book.


This feature is **opt-in** and disabled by default.

```toml
[preprocessor.gitinfo]
contributors = true
```

Contributors are rendered wherever the token appears:

```md
{% contributors %}
```

> [!NOTE]
> Internally, mdbook-gitinfo generates a small CSS file, `gitinfo.css` inside the book’s
> theme override directory and registers it with mdBook, `[output.html.additional-css]`.
> Advanced users may override or extend these styles using their own theme CSS.


### 6.1 Contributor Sources
The source of contributor data is controlled via `contributors-source`.

```toml
[preprocessor.gitinfo]
contributors = true
contributors-source = "git"
```

Supported values:

| Value    | Description                                                                    |
| -------- | ------------------------------------------------------------------------------ |
| `git`    | *(default)* Derive contributors from git history using `git shortlog`.         |
| `file`   | Read contributors from a file at the repository root (e.g. `CONTRIBUTORS.md`). |
| `inline` | Contributors are explicitly listed in the `{% contributors %}` token.          |


If `contributors-source` is omitted, it defaults to `"git"`.


### 6.2 `contributors-source = "git"` (default)

Contributors are inferred from the repository’s git history:

```sh
git shortlog -sne --all
```

The author name is treated as a GitHub username and used to construct:

- Profile link: `https://github.com/<username>`

- Avatar image: `https://github.com/<username>.png`

> [!NOTE]
> This approach assumes commit author names match GitHub usernames.
> While this is common (and recommended), it is a best-effort heuristic.

**Token usage**

```md
{% contributors %}
```

Inline usernames are ignored in this mode:

```md
{% contributors author1 authorN %}
```

A warning is emitted if arguments are provided.

### 6.3 `contributors-source = "file"`

Contributors are read from a file located at the repository root
(the same level as `book.toml` or `README.md`).

```toml
[preprocessor.gitinfo]
contributors = true
contributors-source = "file"
contributors-file = "CONTRIBUTORS.md"
```

Default file name (if omitted):

```md
CONTRIBUTORS.md
```

**Accepted formats**

Each contributor should appear on its own line:

```
name 
name2
name3
```

Bullet prefixes are also accepted:

```
- name 
- name2
- name3
```

Blank lines are ignored.

### 6.4 `contributors-source = "inline"`

Contributors are specified **directly in the token**.

```toml
[preprocessor.gitinfo]
contributors = true
contributors-source = "inline"
```

Usage:
```md
{% contributors name name2 name3 %}
```

If no usernames are provided, nothing is rendered and a warning is emitted.

This mode is intended for:

- curated attribution

- future per-page contributor control

- repositories where git history is not representative

### 6.5 Title, Message, and Exclusions

**Contributor Title**

```toml
[preprocessor.gitinfo]
contributor-title = "Contributors"
```

Default: `"Contributors"`

**Optional Message**

An optional message can be rendered above the avatars.
This supports raw HTML.

```toml
[preprocessor.gitinfo]
contributor-message = "<em>Thanks to all the contributors</em>"
```

**Excluding Contributors**

Usernames can be excluded regardless of source:

```toml
[preprocessor.gitinfo]
exclude-contributors = ["github-actions[bot]", "dependabot"]
```

This is useful for filtering automation accounts or CI bots.

### 6.6 Rendering Behaviour

- The contributors block is rendered as raw HTML

- It is compatible with the HTML renderer

- If contributors are enabled but no valid users are found:

    - nothing is rendered

    - a warning is emitted

## 7. Examples

### Example 1 – Simple Footer

```toml
[preprocessor.gitinfo]
enable = true
footer = true
message.footer = "Updated {{date}} {{sep}} {{hash}}"
align.footer = "center"
```

### Example 2 – Header + Footer with Margins

```toml
[preprocessor.gitinfo]
enable = true
header = true
footer = true
hyperlink = true
font-size = "0.8em"
separator = "||"
branch = "main"

[preprocessor.gitinfo.message]
header = "Last updated: <em>{{date}}</em>"
footer = "branch: {{branch}}{{sep}}commit: {{hash}}"

[preprocessor.gitinfo.align]
header = "left"
footer = "right"

[preprocessor.gitinfo.margin]
header = { top = "2em", bottom = "1em" }
footer = ["2em", "0", "0", "0"]
```