use dashmap::DashMap;
use qrcode::QrCode;
use qrcode::render::svg;
use regex::Regex;
use std::sync::LazyLock;
use url::Url;

fn sanitized_svg(source: String) -> String {
    static RE1: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("width=\"\\d+\" height=\"\\d+\"").unwrap());
    static RE2: LazyLock<Regex> = LazyLock::new(|| Regex::new("<rect.*fill=\"#fff\"/").unwrap());
    let bounded_svg = QrCode::new(source).unwrap().render::<svg::Color>().build();

    let unbounded_svg = RE1.replace(bounded_svg.as_str(), "").to_string();
    let without_background = RE2.replace(unbounded_svg.as_str(), "").to_string();
    without_background
}

pub fn qr_code_as_svg(url: &Url, path: &str, cache: &DashMap<String, String>) -> String {
    let mut url = url.clone();
    url.set_path(path);
    let key = url.to_string();
    if let Some(svg) = cache.get(&key) {
        return (*svg.value()).clone();
    }
    let svg = sanitized_svg(url.to_string());
    cache.insert(key, svg.clone());

    svg
}
