use crate::ode_solve::{self, Simulator};

pub struct RungeKuttaSimulator {}

#[allow(dead_code)]
impl RungeKuttaSimulator {
    pub fn new() -> Self {
        RungeKuttaSimulator {}
    }
}

impl Simulator for RungeKuttaSimulator {
    fn simulation_type(&self) -> String {
        String::from("RungeKutta")
    }

    fn get_next(&self, time: f64, step: f64, (q, p): (f64, f64)) -> (f64, f64) {
        let k1_q = ode_solve::hamiltonian_p(time, (q, p)) * step;
        let k1_p = -ode_solve::hamiltonian_q(time, (q, p)) * step;

        let k2_q = ode_solve::hamiltonian_p(
            time + step / 2.0,
            (q + k1_q / 2.0, p + k1_p / 2.0),
        ) * step;
        let k2_p = -ode_solve::hamiltonian_q(
            time + step / 2.0,
            (q + k1_q / 2.0, p + k1_p / 2.0),
        ) * step;

        let k3_q = ode_solve::hamiltonian_p(
            time + step / 2.0,
            (q + k2_q / 2.0, p + k2_p / 2.0),
        ) * step;
        let k3_p = -ode_solve::hamiltonian_q(
            time + step / 2.0,
            (q + k2_q / 2.0, p + k2_p / 2.0),
        ) * step;

        let k4_q = ode_solve::hamiltonian_p(time + step, (q + k3_q, p + k3_p))
            * step;
        let k4_p = -ode_solve::hamiltonian_q(time + step, (q + k3_q, p + k3_p))
            * step;

        (
            q + (k1_q + 2.0 * k2_q + 2.0 * k3_q + k4_q) / 6.0,
            p + (k1_p + 2.0 * k2_p + 2.0 * k3_p + k4_p) / 6.0,
        )
    }
}
