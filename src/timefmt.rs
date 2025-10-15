use chrono::{DateTime, FixedOffset, Local, Utc};

#[derive(Clone, Debug)]
pub enum TzMode {
    Local,
    Utc,
    Source,
    Fixed(FixedOffset),
}

impl TzMode {
    pub fn parse(s: Option<&str>) -> Self {
        let raw = s.unwrap_or("local").trim().to_ascii_lowercase();
        match raw.as_str() {
            "local"  => Self::Local,
            "utc"    => Self::Utc,
            "source" => Self::Source,
            other if other.starts_with("fixed:") => {
                let off = &other["fixed:".len()..]; // ±HH:MM
                match off.split_once(':').and_then(|(h,m)| {
                    let (sign, hh) = match h.chars().next()? {
                        '+' => (1, &h[1..]),
                        '-' => (-1, &h[1..]),
                        _ => return None,
                    };
                    let h: i32 = hh.parse().ok()?;
                    let m: i32 = m.parse().ok()?;
                    Some(sign * (h * 3600 + m * 60))
                }).and_then(FixedOffset::east_opt) {
                    Some(fo) => Self::Fixed(fo),
                    None => {
                        eprintln!("[gitinfo] Warning: invalid fixed offset '{off}', using 'local'");
                        Self::Local
                    }
                }
            }
            other => {
                eprintln!("[gitinfo] Warning: unrecognised timezone '{other}', using 'local'");
                Self::Local
            }
        }
    }
}

/// Format a commit datetime string (RFC3339) using a target timezone mode.
/// Offset is applied but only printed if the user's format includes %z/%:z/%Z.
pub fn format_commit_datetime(
    raw_rfc3339: &str,
    tz_opt: Option<&str>,
    date_fmt: &str,
    time_fmt: &str,
) -> String {
    let dt_src: DateTime<FixedOffset> = match DateTime::parse_from_rfc3339(raw_rfc3339) {
        Ok(d) => d,
        Err(_) => return "unknown".to_string(),
    };

    let dt_fixed: DateTime<FixedOffset> = match TzMode::parse(tz_opt) {
        TzMode::Utc     => dt_src.with_timezone(&Utc).fixed_offset(),
        TzMode::Source  => dt_src,
        TzMode::Fixed(o)=> dt_src.with_timezone(&o).fixed_offset(),
        TzMode::Local   => dt_src.with_timezone(&Local).fixed_offset(),
    };

    let fmt = format!("{} {}", date_fmt, time_fmt).trim().to_string();
    dt_fixed.format(&fmt).to_string()
}
