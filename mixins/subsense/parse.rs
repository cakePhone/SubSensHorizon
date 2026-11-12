fn subsense_apply(file: &mut OptionsFile, raw: &std::collections::HashMap<String, String>) {
    if file.try_apply(raw).is_ok() {
        if let Some(resolution) = raw.get("fullscreenResolution") {
            file.fullscreen_resolution = Some(resolution.clone());
        }
        if let Some(text) = raw.get("mouseSensitivity")
            && let Some(sensitivity) = crate::options_fixes::java_parse_double(text)
            && sensitivity.is_finite()
            && (subsense_runtime::MIN..=subsense_runtime::MAX).contains(&sensitivity)
            && let Some(i) = OptionsFile::index("mouseSensitivity")
        {
            file.values[i] = V::Double(sensitivity);
        }
    }
}
