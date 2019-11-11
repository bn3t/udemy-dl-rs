use crate::result::Result;
use failure::format_err;
use serde_json::Value;

/// Returns a cross-platform-filename-safe version of any string.
///
/// This is used internally to generate app data directories based on app
/// name/author. App developers can use it for consistency when dealing with
/// file system operations.
///
/// Do not apply this function to full paths, as it will sanitize '/' and '\';
/// it should only be used on directory or file names (i.e. path segments).
pub fn sanitize(component: &str) -> String {
    let mut buf = String::with_capacity(component.len());
    for (i, c) in component.chars().enumerate() {
        let is_lower = 'a' <= c && c <= 'z';
        let is_upper = 'A' <= c && c <= 'Z';
        let is_letter = is_upper || is_lower;
        let is_number = '0' <= c && c <= '9';
        let is_space = c == ' ';
        let is_hyphen = c == '-';
        let is_underscore = c == '_';
        let is_period = c == '.' && i != 0; // Disallow accidentally hidden folders
        let is_valid =
            is_letter || is_number || is_space || is_hyphen || is_underscore || is_period;
        if is_valid {
            buf.push(c);
        } else {
            buf.push_str("_");
        }
    }
    buf
}

pub fn calculate_download_speed(total: u64, elapsed: u64) -> f64 {
    (total * 1000u64 / elapsed) as f64 / 1024.0 / 1024.0
}

pub fn json_get_string<'a, 'b>(value: &'a Value, key: &'b str) -> Result<&'a str> {
    value
        .get(key)
        .ok_or_else(|| format_err!("Error parsing json ({})", key))?
        .as_str()
        .ok_or_else(|| format_err!("Error parsing json ({})", key))
}

pub fn json_get_u64(value: &Value, key: &str) -> Result<u64> {
    value
        .get(key)
        .ok_or_else(|| format_err!("Error parsing json ({})", key))?
        .as_u64()
        .ok_or_else(|| format_err!("Error parsing json ({})", key))
}

#[cfg(test)]
mod test_udemy_helper {
    use super::*;

    #[test]
    fn sanitize_normal() {
        let actual = sanitize("the-filename.mp4");

        assert_eq!(actual, "the-filename.mp4");
    }

    #[test]
    fn sanitize_illegal() {
        let actual =
            sanitize(r#"087 Styling & Positioning our Badge with "absolute" and "relative".mp4"#);

        assert_eq!(
            actual,
            "087 Styling _ Positioning our Badge with _absolute_ and _relative_.mp4"
        );
    }

    #[test]
    fn test_calculate_download_speed() {
        let actual = calculate_download_speed(1024u64 * 1024u64, 1000);

        assert_eq!(actual, 1.0);
    }
}
