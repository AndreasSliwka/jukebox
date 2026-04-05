use askama::filters::Safe;
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

const TRAILING_WHITESPACE: &str = r"\s+$";

#[askama::filter_fn]
pub fn make_trailing_whitespace_nbsp(
    value: &&String,
    _env: &dyn askama::Values,
) -> askama::Result<Safe<String>> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(TRAILING_WHITESPACE).unwrap());
    Ok(Safe(RE.replace(value.as_str(), "&nbsp;").to_string()))
}
