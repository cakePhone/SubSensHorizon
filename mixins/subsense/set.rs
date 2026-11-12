fn subsense_set(file: &mut OptionsFile, key: &str, value: &V) -> std::ops::ControlFlow<()> {
    if key == "mouseSensitivity" && subsense::set_value(file, value) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
