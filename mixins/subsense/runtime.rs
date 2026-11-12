// SubSens' pure range logic.
//
// Vanilla Minecraft maps the sensitivity slider to `0.0..=1.0`. SubSens
// extends the bottom of that range to `-1/3`, the value at which the game's
// `(sensitivity * 0.6 + 0.2)^3` curve reaches true zero.
//
// This file is both the standalone Cargo library and the support module
// injected into `options_file.rs`, so it must stay free of game types.

/// The lowest sensitivity the slider can reach: true zero camera movement.
pub const MIN: f64 = -1.0 / 3.0;

/// The vanilla maximum, unchanged.
pub const MAX: f64 = 1.0;

/// The slider position (`0..=1`) for a sensitivity value.
pub fn slider_from_value(value: f64) -> f32 {
    (((value - MIN) / (MAX - MIN)).clamp(0.0, 1.0)) as f32
}

/// The sensitivity value for a slider position.
///
/// When the widget reports the position already derived from `current`, the
/// exact loaded value is kept. That is what lets `options.txt` carry more
/// precision than the slider itself can represent.
pub fn value_from_slider(current: f64, slider: f32) -> f64 {
    let slider = f64::from(slider.clamp(0.0, 1.0));
    if f64::from(slider_from_value(current)).to_bits() == slider.to_bits() {
        current
    } else {
        MIN + slider * (MAX - MIN)
    }
}

/// The percentage shown beside the slider (`Math.round(value * 200.0)`).
pub fn percentage(value: f64) -> i64 {
    (value * 200.0).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bottom_of_the_slider_is_true_zero() {
        assert_eq!(slider_from_value(MIN), 0.0);
        assert_eq!(value_from_slider(0.5, 0.0), MIN);
        // The game's f32 curve reaches zero to within float rounding.
        let curve = (MIN as f32 * 0.6 + 0.2).powi(3);
        assert!(curve.abs() < 1e-20, "curve was {curve}");
    }

    #[test]
    fn the_top_and_middle_are_unchanged() {
        assert_eq!(slider_from_value(MAX), 1.0);
        assert_eq!(value_from_slider(0.5, 1.0), MAX);
        assert_eq!(slider_from_value(0.0), 0.25);
        assert_eq!(slider_from_value(0.5), 0.625);
    }

    #[test]
    fn an_untouched_slider_keeps_the_exact_loaded_value() {
        for value in [-0.015_399_561_858_452_84, 0.000_669_61, 0.0, 0.5, 1.0] {
            let position = slider_from_value(value);
            assert_eq!(value_from_slider(value, position), value);
        }
    }

    #[test]
    fn moving_the_slider_uses_the_extended_range() {
        let position = slider_from_value(0.5);
        let moved = value_from_slider(0.5, position + 0.01);
        assert!(moved > 0.5);
        assert_eq!(value_from_slider(0.5, 2.0), MAX);
        assert_eq!(value_from_slider(0.5, -1.0), MIN);
    }

    #[test]
    fn percentages_match_the_java_mod() {
        assert_eq!(percentage(MIN), -67);
        assert_eq!(percentage(0.0), 0);
        assert_eq!(percentage(0.5), 100);
        assert_eq!(percentage(1.0), 200);
    }
}
