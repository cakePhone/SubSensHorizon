fn subsense_label(this: Opt, s: &Settings) -> std::ops::ControlFlow<Text> {
    if this == Opt::Sensitivity {
        std::ops::ControlFlow::Break(subsense::label(this.caption(), s.sensitivity))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
