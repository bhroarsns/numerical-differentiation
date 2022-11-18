use crate::ode_solve::{self, Simulator};

pub struct LeapFlogSimulator {}

#[allow(dead_code)]
impl LeapFlogSimulator {
    pub fn new() -> Self {
        LeapFlogSimulator {}
    }
}

impl Simulator for LeapFlogSimulator {
    fn simulation_type(&self) -> String {
        String::from("LeapFlog")
    }

    fn get_next(&self, time: f64, step: f64, (q, p): (f64, f64)) -> (f64, f64) {
        let half_p = p - ode_solve::hamiltonian_q(time, (q, p)) * step / 2.0;
        let new_q = q + step * ode_solve::hamiltonian_p(time, (q, half_p));
        let new_p =
            half_p - ode_solve::hamiltonian_q(time, (new_q, half_p)) * step / 2.0;
        (new_q, new_p)
    }
}
