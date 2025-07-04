
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


An [mdBook](https://github.com/rust-lang/mdBook) preprocessor that injects Git metadata (e.g., commit hash, date, tag) into your rendered book. This is useful for displaying build provenance or version information.

## Features

- Injects the latest Git commit information for each/per chapter/subchapter.
- Fully configurable output format using template variables.
- Supports date and time formatting.
- Renders a styled footer below each chapter with Git metadata.
- Supports the `html` renderer.

## Installation

From package manager:

```sh
cargo install mdbook-gitinfo
```

Clone this repository and install it using Cargo:

```sh
cargo install --path .
```

Make sure the binary (`mdbook-gitinfo`) is in your `PATH`.

## Configuration

Add the following to your book.toml:

```toml
[preprocessor.gitinfo]
enable = true
template = "Date: {{date}}{{sep}}commit: {{hash}}"
separator = " • "
font-size = "0.8em"
date-format = "%Y-%m-%d"
time-format = "%H:%M:%S"
```

> [!NOTE]
> date and time formatting use chrono format specifiers
> | Format | Meaning                      | Example  |
> | ------ | ---------------------------- | -------- |
> | `%Y`   | Year with century            | `2025`   |
> | `%y`   | Year without century (00-99) | `25`     |
> | `%m`   | Month number (01-12)         | `06`     |
> | `%b`   | Abbreviated month name       | `Jun`    |
> | `%B`   | Full month name              | `June`   |
> | `%d`   | Day of month (01-31)         | `24`     |
> | `%e`   | Day of month, space-padded   | `24`     |
> | `%a`   | Abbreviated weekday          | `Mon`    |
> | `%A`   | Full weekday name            | `Monday` |
> | `%j`   | Day of year (001–366)        | `176`    |
>
> | Format | Meaning                        | Example  |
> | ------ | ------------------------------ | -------- |
> | `%H`   | Hour (00-23)                   | `14`     |
> | `%I`   | Hour (01-12)                   | `02`     |
> | `%p`   | AM/PM                          | `PM`     |
> | `%M`   | Minute (00-59)                 | `05`     |
> | `%S`   | Second (00-60, leap-sec aware) | `09`     |
> | `%f`   | Microseconds (000000-999999)   | `123456` |
> | `%z`   | +hhmm timezone offset          | `+0100`  |
> | `%:z`  | +hh\:mm timezone offset        | `+01:00` |
> | `%Z`   | Time zone name                 | `BST`    |


## Template Variables
You can use the following placeholders in the template string:

- `{{hash}}`: Short commit hash (`git rev-parse --short HEAD`)

- `{{long}}`: Full commit hash

- `{{tag}}`: Git tag (from `git describe`)

- `{{date}}`: Timestamp of the latest commit, formatted

- `{{sep}}`: Separator (defaults to " • ")

## Example Output

With the above configuration, this footer will be injected:

```html
<footer>
  <span class="gitinfo-footer" style="font-size:0.8em;...">
    Date: 2025-06-23 16:19:28 • commit: 2160ec5
  </span>
</footer>
```

## Compatibility

- `mdbook-gitinfo` is tested with mdbook 0.4.x.

- Only the html renderer is supported.

## License

[Apache-2.0](LICENSE.md)

## Author

[CompEng0001](https://github.com/CompEng0001)