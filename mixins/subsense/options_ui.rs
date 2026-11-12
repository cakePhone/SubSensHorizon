// The sensitivity slider and its label.
//
// The slider position stays `0..=1`; SubSens only changes the value range it
// represents, from `-1/3..=1`.

use super::Text;
use crate::options_file::subsense_runtime;

/// The slider position for the current sensitivity.
pub(crate) fn slider(value: f64) -> f32 {
    subsense_runtime::slider_from_value(value)
}

/// The sensitivity for a slider position, keeping an exact untouched value.
pub(crate) fn value(current: f64, slider: f32) -> f64 {
    subsense_runtime::value_from_slider(current, slider)
}

/// `options.sensitivity`'s label: a percentage, or `Max` at the top.
pub(crate) fn label(caption: &str, value: f32) -> Text {
    if value >= 1.0 {
        super::generic(caption, super::tr("options.sensitivity.max"))
    } else {
        let percentage = subsense_runtime::percentage(f64::from(value));
        Text::translate(
            "options.percent_value",
            &[super::tr(caption), Text::literal(percentage.to_string())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Opt, Settings};
    use crate::options_file::subsense_runtime::{MAX, MIN};

    #[test]
    fn the_slider_spans_the_extended_range() {
        let mut settings = Settings::default();
        settings.sensitivity = MIN as f32;
        assert_eq!(Opt::Sensitivity.slider_position(&settings), Some(0.0));
        settings.sensitivity = MAX as f32;
        assert_eq!(Opt::Sensitivity.slider_position(&settings), Some(1.0));
    }

    #[test]
    fn moving_the_sensitivity_slider_uses_the_extended_range() {
        let mut settings = Settings::default();
        settings.sensitivity = 0.5;
        Opt::Sensitivity.set_slider(&mut settings, 0.0);
        assert!((settings.sensitivity - MIN as f32).abs() < 1e-6);
        Opt::Sensitivity.set_slider(&mut settings, 1.0);
        assert_eq!(settings.sensitivity, 1.0);
    }

    #[test]
    fn an_untouched_slider_keeps_the_loaded_value() {
        let mut settings = Settings::default();
        settings.sensitivity = 0.000_669_61;
        let position = Opt::Sensitivity.slider_position(&settings).unwrap();
        Opt::Sensitivity.set_slider(&mut settings, position);
        assert_eq!(settings.sensitivity, 0.000_669_61);
    }

    #[test]
    fn settings_round_trip_a_negative_sensitivity() {
        let mut settings = Settings::default();
        settings.sensitivity = -0.2;
        let text = settings.serialize();
        assert!(text.contains("mouseSensitivity:-0.2"), "{text}");
        assert_eq!(Settings::parse(&text).sensitivity, -0.2);
    }

    #[test]
    fn settings_keep_an_exact_loaded_sensitivity() {
        let text = "version:9999\nmouseSensitivity:-0.01539956185845284\n";
        let settings = Settings::parse(text);
        assert_eq!(settings.sensitivity, -0.01539956185845284_f32);
        assert!(
            settings
                .serialize()
                .contains("mouseSensitivity:-0.01539956185845284")
        );
    }
}
