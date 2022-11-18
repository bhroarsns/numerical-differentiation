use crate::ode_solve::Simulator;

pub struct SymplecticSimulator {}

#[allow(dead_code)]
impl SymplecticSimulator {
    pub fn new() -> Self {
        SymplecticSimulator {}
    }
}

impl Simulator for SymplecticSimulator {
    fn simulation_type(&self) -> String {
        String::from("Symplectic")
    }

    fn get_next(&self, _: f64, step: f64, (q, p): (f64, f64)) -> (f64, f64) {
        (
            q + step * p,
            (1.0 - step * step) * p - step * q,
        )
    }
}
