use crate::config::{AlignSetting, GitInfoConfig, MarginConfig, MarginSetting};

pub fn resolve_messages(cfg: &GitInfoConfig) -> (String, String) {
    let default = "{{date}}{{sep}}commit: {{hash}}".to_string();
    let both = cfg.message.as_ref().and_then(|m| m.both.clone());

    let header = cfg.message.as_ref().and_then(|m| m.header.clone())
        .or_else(|| both.clone())
        .or_else(|| cfg.template.clone())
        .unwrap_or_else(|| default.clone());

    let footer = cfg.message.as_ref().and_then(|m| m.footer.clone())
        .or_else(|| both.clone())
        .or_else(|| cfg.template.clone())
        .unwrap_or(default);

    (header, footer)
}

pub fn resolve_align(a: &Option<AlignSetting>) -> (String, String) {
    match a {
        Some(AlignSetting::One(s)) => (s.clone(), s.clone()),
        Some(AlignSetting::Split { header, footer, both }) => {
            let both_v = both.clone().unwrap_or_else(|| "center".to_string());
            let h = header.clone().unwrap_or_else(|| both_v.clone());
            let f = footer.clone().unwrap_or_else(|| both_v);
            (h, f)
        }
        None => ("center".into(), "center".into()),
    }
}

fn margin_from_setting(ms: &MarginSetting, fallback: [&str; 4]) -> [String; 4] {
    match ms {
        MarginSetting::One(v) => [v.clone(), v.clone(), v.clone(), v.clone()],
        MarginSetting::Quad(vs) => match vs.len() {
            0 => fallback.map(|s| s.to_string()),
            1 => {
                let v = vs[0].clone();
                [v.clone(), v.clone(), v.clone(), v]
            }
            2 => {
                let vtb = vs[0].clone();
                let vrl = vs[1].clone();
                [vtb.clone(), vrl.clone(), vtb, vrl]
            }
            3 => {
                let t = vs[0].clone();
                let rl = vs[1].clone();
                let b = vs[2].clone();
                [t, rl.clone(), b, rl]
            }
            _ => [vs[0].clone(), vs[1].clone(), vs[2].clone(), vs[3].clone()],
        },
        MarginSetting::Sides { top, right, bottom, left } => [
            top.clone().unwrap_or_else(|| fallback[0].to_string()),
            right.clone().unwrap_or_else(|| fallback[1].to_string()),
            bottom.clone().unwrap_or_else(|| fallback[2].to_string()),
            left.clone().unwrap_or_else(|| fallback[3].to_string()),
        ],
    }
}

pub fn resolve_margins(m: &Option<MarginConfig>) -> ([String;4], [String;4]) {
    let default_header = ["0", "0", "2em", "0"];
    let default_footer = ["0", "0", "2em", "0"];

    let both = m.as_ref().and_then(|mm| mm.both.as_ref());
    let base = both.map(|b| margin_from_setting(b, ["0","0","0","0"]))
                   .unwrap_or(["0".into(),"0".into(),"0".into(),"0".into()]);

    let header = m.as_ref().and_then(|mm| mm.header.as_ref())
        .map(|h| margin_from_setting(h, [&base[0], &base[1], &base[2], &base[3]]))
        .unwrap_or_else(|| default_header.map(|s| s.to_string()));

    let footer = m.as_ref().and_then(|mm| mm.footer.as_ref())
        .map(|f| margin_from_setting(f, [&base[0], &base[1], &base[2], &base[3]]))
        .unwrap_or_else(|| default_footer.map(|s| s.to_string()));

    (header, footer)
}
