mod bisection_two;
mod differential;
mod euler;
mod leap_flog;
mod newton;
mod newton_two;
mod ode_solve;
mod rev_euler;
mod runge_kutta;
mod secondary;
mod symplectic;

fn main() {
    //differential::plot().unwrap();
    //secondary::plot().unwrap();

    /*
    newton::find_root(0.0).unwrap();
    newton::find_root(1.0).unwrap();
    newton::find_root(1.3).unwrap();

    bisection_two::find_root(0.0).unwrap();
    bisection_two::find_root(1.0).unwrap();
    bisection_two::find_root(1.3).unwrap();
    bisection_two::find_root(10.0).unwrap();
    bisection_two::find_root(-10.0).unwrap();

    newton_two::find_root(0.0).unwrap();
    newton_two::find_root(1.0).unwrap();
    newton_two::find_root(1.3).unwrap();
    newton_two::find_root(10.0).unwrap();
    newton_two::find_root(-10.0).unwrap();

    */
    ode_solve::run(0.1, 100, (1.0, 0.0)).unwrap();
    //ode_solve::plot_energy(10, 1000, (100.0, 0.0)).unwrap();
}
