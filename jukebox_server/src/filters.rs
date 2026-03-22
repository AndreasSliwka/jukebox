use regex::Regex;
use std::sync::LazyLock;

// 2026-04-21T19:04:57.795Z
const TIME_FROM_STAMP: &str = r"^\d\d\d\d-\d\d-\d\d[ T](\d\d:\d\d)";

#[askama::filter_fn]
pub fn time_from_timestamp(value: &&String, _env: &dyn askama::Values) -> askama::Result<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(TIME_FROM_STAMP).unwrap());
    if let Some(captures) = RE.captures(value.as_str()) {
        return Ok(captures[1].to_string());
    }
    Ok(String::from(""))
}
