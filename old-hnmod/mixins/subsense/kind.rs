fn subsense_kind(value: Kind, this: Opt, s: &Settings) -> Kind {
    if this == Opt::Sensitivity {
        Kind::Slider(subsense::slider(f64::from(s.sensitivity)))
    } else {
        value
    }
}
