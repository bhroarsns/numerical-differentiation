use crate::ode_solve::Simulator;

pub struct ReverseEulerSimulator {}

#[allow(dead_code)]
impl ReverseEulerSimulator {
    pub fn new() -> Self {
        ReverseEulerSimulator {}
    }
}

impl Simulator for ReverseEulerSimulator {
    fn simulation_type(&self) -> String {
        String::from("ReverseEuler")
    }

    fn get_next(&self, _: f64, step: f64, (q, p): (f64, f64)) -> (f64, f64) {
        (
            (q + step * p) / (1.0 + step * step),
            (p - step * q) / (1.0 + step * step),
        )
    }
}
