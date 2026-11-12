fn subsense_set_slider(this: Opt, s: &mut Settings, v: f32) -> std::ops::ControlFlow<()> {
    if this == Opt::Sensitivity {
        s.sensitivity = subsense::value(f64::from(s.sensitivity), v) as f32;
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
