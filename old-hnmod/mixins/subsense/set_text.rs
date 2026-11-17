fn subsense_set_text(
    file: &mut OptionsFile,
    key: &str,
    text: &str,
) -> std::ops::ControlFlow<Result<(), String>> {
    match subsense::set_text(file, key, text) {
        Some(result) => std::ops::ControlFlow::Break(result),
        None => std::ops::ControlFlow::Continue(()),
    }
}
