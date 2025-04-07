pub fn exponential_curve(initial_value: f64, rate: f64, time: f64) -> f64 {
    // https://en.wikipedia.org/wiki/Exponential_growth
    initial_value * (1.0 + rate).powf(time)
}
