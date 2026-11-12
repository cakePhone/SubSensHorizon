// `options.txt` support for SubSens.
//
// Horizon's vanilla `mouseSensitivity` option uses the `UnitDouble` codec,
// which refuses values below `0.0` when loading, setting and saving. SubSens
// widens that one option to `-1/3..=1` without touching any other option.

use super::{OptionsFile, V};
use crate::options_file::subsense_runtime::{MAX, MIN};

/// The index of `mouseSensitivity` in the options table.
fn index() -> Option<usize> {
    OptionsFile::index("mouseSensitivity")
}

/// The raw stored sensitivity, if the option is a double.
fn stored(file: &OptionsFile) -> Option<f64> {
    match file.values.get(index()?)? {
        V::Double(value) => Some(*value),
        _ => None,
    }
}

/// Set `mouseSensitivity` directly, bypassing the `UnitDouble` validation.
/// Returns `false` for any other option or an out-of-range value.
pub(crate) fn set_value(file: &mut OptionsFile, value: &V) -> bool {
    let Some(i) = index() else { return false };
    let V::Double(value) = value else {
        return false;
    };
    if !value.is_finite() || *value < MIN || *value > MAX {
        return false;
    }
    file.values[i] = V::Double(*value);
    true
}

/// `OptionsFile::set_text` for `mouseSensitivity`, including negative values.
/// `None` means the key is not ours; `Some` is the result the caller returns.
pub(crate) fn set_text(
    file: &mut OptionsFile,
    key: &str,
    text: &str,
) -> Option<Result<(), String>> {
    if key != "mouseSensitivity" {
        return None;
    }
    let Some(value) = crate::options_fixes::java_parse_double(text) else {
        return Some(Err(refused(text)));
    };
    if !value.is_finite() || value < MIN || value > MAX {
        return Some(Err(refused(text)));
    }
    let Some(i) = index() else {
        return Some(Err(refused(text)));
    };
    file.values[i] = V::Double(value);
    Some(Ok(()))
}

/// Keep the `mouseSensitivity` line when the stored value is negative.
///
/// `Spec::Unit`'s encoder refuses negative values, so `serialize` would omit
/// the line entirely. Append it with the exact stored double instead.
pub(crate) fn serialize(mut out: String, file: &OptionsFile) -> String {
    let Some(value) = stored(file) else {
        return out;
    };
    if !value.is_sign_negative() {
        return out;
    }
    if out
        .lines()
        .any(|line| line.starts_with("mouseSensitivity:"))
    {
        let mut kept = String::with_capacity(out.len());
        for line in out.lines() {
            if !line.starts_with("mouseSensitivity:") {
                kept.push_str(line);
                kept.push('\n');
            }
        }
        out = kept;
    }
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("mouseSensitivity:");
    out.push_str(&crate::java_text::double_to_string(value));
    out.push('\n');
    out
}

fn refused(text: &str) -> String {
    format!("mouseSensitivity refuses {text:?}; it accepts a number from {MIN} to {MAX}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options_file::OptionsFile;

    fn parsed(text: &str) -> OptionsFile {
        OptionsFile::parse(text)
    }

    #[test]
    fn negative_sensitivity_loads_and_saves_exactly() {
        let text = "version:9999\nmouseSensitivity:-0.01539956185845284\n";
        let file = parsed(text);
        assert_eq!(file.double("mouseSensitivity"), -0.01539956185845284);
        let saved = file.serialize();
        assert!(
            saved.contains("mouseSensitivity:-0.01539956185845284"),
            "{saved}"
        );
        assert_eq!(
            parsed(&saved).double("mouseSensitivity"),
            -0.01539956185845284
        );
    }

    #[test]
    fn negative_sensitivity_can_be_set_directly_and_as_text() {
        let mut file = OptionsFile::default();
        assert!(set_value(&mut file, &V::Double(-0.25)));
        assert_eq!(file.double("mouseSensitivity"), -0.25);
        assert!(!set_value(&mut file, &V::Double(-0.5)));
        assert!(
            set_text(&mut file, "mouseSensitivity", "-0.2")
                .unwrap()
                .is_ok()
        );
        assert_eq!(file.double("mouseSensitivity"), -0.2);
        assert!(
            set_text(&mut file, "mouseSensitivity", "-0.5")
                .unwrap()
                .is_err()
        );
        assert!(set_text(&mut file, "gamma", "-0.2").is_none());
    }

    #[test]
    fn positive_sensitivity_keeps_one_vanilla_line() {
        let file = parsed("version:9999\nmouseSensitivity:0.5\n");
        assert_eq!(file.serialize().matches("mouseSensitivity:").count(), 1);
    }

    #[test]
    fn negative_zero_keeps_its_sign() {
        let file = parsed("version:9999\nmouseSensitivity:-0.0\n");
        assert!(file.serialize().contains("mouseSensitivity:-0.0"));
    }
}
