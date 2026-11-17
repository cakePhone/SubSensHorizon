//! SubSens for the Horizon mod API: sensitivity down to true zero.

use mod_api::{
    self, CalculatePlayerTurnEvent, ClientHost, Hooks, Registrar, RegistrationContext, SettingSpec,
};

/// The lowest sensitivity the slider can reach: true zero camera movement.
const MIN: f64 = -1.0 / 3.0;

/// The vanilla maximum, unchanged.
const MAX: f64 = 1.0;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::tooltip(
            SettingSpec::toggle("enabled", "subsense.setting.enabled", false),
            "subsense.setting.enabled.tooltip",
        ));
        let _ = registrar.register_setting(SettingSpec::tooltip(
            SettingSpec::range(
                "sensitivity",
                "subsense.setting.sensitivity",
                MIN,
                MAX,
                0.0001,
                0.5,
            ),
            "subsense.setting.sensitivity.tooltip",
        ));
    }

    fn calculate_player_turn(&self, host: &dyn ClientHost, event: &mut CalculatePlayerTurnEvent) {
        let enabled = host.setting_bool("enabled").unwrap_or(false);
        let sensitivity = host.setting_f64("sensitivity").unwrap_or(0.5);
        if let Some(value) = effective_sensitivity(enabled, sensitivity) {
            event.mouse_sensitivity = value;
        }
    }
}

/// The sensitivity this turn uses, or `None` to leave vanilla's alone.
fn effective_sensitivity(enabled: bool, sensitivity: f64) -> Option<f64> {
    enabled.then(|| sensitivity.clamp(MIN, MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_leaves_vanilla_alone() {
        assert_eq!(effective_sensitivity(false, -0.2), None);
    }

    #[test]
    fn enabled_uses_the_extended_range() {
        assert_eq!(effective_sensitivity(true, -0.2), Some(-0.2));
        assert_eq!(effective_sensitivity(true, -1.0), Some(MIN));
        assert_eq!(effective_sensitivity(true, 2.0), Some(MAX));
    }

    #[test]
    fn the_bottom_is_true_zero() {
        let curve = (MIN as f32 * 0.6 + 0.2).powi(3);
        assert!(curve.abs() < 1e-20, "curve was {curve}");
    }
}
