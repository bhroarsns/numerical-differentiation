use crate::ode_solve::{self, Simulator};

pub struct EulerSimulator {}

#[allow(dead_code)]
impl EulerSimulator {
    pub fn new() -> Self {
        EulerSimulator {}
    }
}

impl Simulator for EulerSimulator {
    fn simulation_type(&self) -> String {
        String::from("Euler")
    }

    fn get_next(&self, time: f64, step: f64, state: (f64, f64)) -> (f64, f64) {
        let new_q = state.0 + ode_solve::hamiltonian_p(time, state) * step;
        let new_p = state.1 - ode_solve::hamiltonian_q(time, state) * step;
        (new_q, new_p)
    }
}
