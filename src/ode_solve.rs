use plotters::prelude::*;

type Data = Vec<(f64, (f64, f64), f64)>;

pub trait Simulator {
    fn simulation_type(&self) -> String;
    fn get_next(&self, time: f64, step: f64, state: (f64, f64)) -> (f64, f64);

    fn simluate(
        &self,
        step: f64,
        duration: u64,
        initial_state: (f64, f64),
    ) -> (
        Data,
        std::ops::Range<f64>,
        std::ops::Range<f64>,
        std::ops::Range<f64>,
    ) {
        let mut time = 0.0;
        let mut result: Data = Vec::new();

        let mut current_state = initial_state;
        let mut qmax = current_state.0;
        let mut qmin = current_state.0;
        let mut pmax = current_state.1;
        let mut pmin = current_state.1;
        let mut emax = hamiltonian(0.0, current_state);
        let mut emin = hamiltonian(0.0, current_state);

        result.push((time, current_state, hamiltonian(0.0, current_state)));

        for _ in 0..duration {
            time += step;
            current_state = self.get_next(time, step, current_state);

            if current_state.0 > qmax {
                qmax = current_state.0;
            }
            if current_state.0 < qmin {
                qmin = current_state.0;
            }
            if current_state.1 > pmax {
                pmax = current_state.1;
            }
            if current_state.1 < pmin {
                pmin = current_state.1;
            }
            if hamiltonian(0.0, current_state) < emin {
                emin = hamiltonian(0.0, current_state)
            }
            if hamiltonian(0.0, current_state) > emax {
                emax = hamiltonian(0.0, current_state)
            }
            result.push((time, current_state, hamiltonian(0.0, current_state)));
        }

        (
            result,
            std::ops::Range {
                start: qmin - 0.1,
                end: qmax + 0.1,
            },
            std::ops::Range {
                start: pmin - 0.1,
                end: pmax + 0.1,
            },
            std::ops::Range {
                start: emin - 0.1,
                end: emax + 0.1,
            },
        )
    }
}

#[allow(dead_code)]
pub fn run(
    step: f64,
    duration: u64,
    initial_state: (f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    let es = crate::euler::EulerSimulator::new();
    plot(es, step, duration, initial_state)?;
    let rks = crate::runge_kutta::RungeKuttaSimulator::new();
    plot(rks, step, duration, initial_state)?;
    let res = crate::rev_euler::ReverseEulerSimulator::new();
    plot(res, step, duration, initial_state)?;
    let lfs = crate::leap_flog::LeapFlogSimulator::new();
    plot(lfs, step, duration, initial_state)?;
    let ss = crate::symplectic::SymplecticSimulator::new();
    plot(ss, step, duration, initial_state)?;

    Ok(())
}

#[allow(dead_code)]
pub fn plot<T: Simulator>(
    simulator: T,
    step: f64,
    duration: u64,
    initial_state: (f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    let dirname = format!(
        "./ode_solve/{}-{}-{}-{}",
        initial_state.0,
        initial_state.1,
        step,
        duration
    );
    if std::fs::create_dir(&dirname).is_ok() {};

    let filename = format!(
        "./ode_solve/{}-{}-{}-{}/{}.svg",
        initial_state.0,
        initial_state.1,
        step,
        duration,
        simulator.simulation_type()
    );

    println!(
        "begin simulating with {} simulator",
        simulator.simulation_type()
    );
    println!(
        "step: {}, initial state: {:?}",
        step,
        initial_state
    );

    let (result, qrange, prange, erange) = simulator.simluate(step, duration, initial_state);
    let trange = std::ops::Range {
        start: 0.0,
        end: duration as f64 * step,
    };

    let root = SVGBackend::new(&filename, (4096, 3072)).into_drawing_area();
    let roots = root.split_evenly((2, 2));

    for i in roots.iter().take(4) {
        i.fill(&WHITE)?;
    }

    let mut cc = ChartBuilder::on(&roots[0])
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_2d(trange.clone(), qrange.clone())?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .x_desc("t")
        .y_desc("q")
        .draw()?;

    cc.draw_series(LineSeries::new(
        result.clone().into_iter().map(|(t, (q, _), _)| (t, q)),
        &BLACK,
    ))?;

    let mut cc = ChartBuilder::on(&roots[1])
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_2d(trange.clone(), prange.clone())?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .x_desc("t")
        .y_desc("p")
        .draw()?;

    cc.draw_series(LineSeries::new(
        result.clone().into_iter().map(|(t, (_, p), _)| (t, p)),
        &BLACK,
    ))?;

    let mut cc = ChartBuilder::on(&roots[2])
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_2d(qrange, prange)?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .x_desc("q")
        .y_desc("p")
        .draw()?;

    cc.draw_series(LineSeries::new(
        result.clone().into_iter().map(|(_, (q, p), _)| (q, p)),
        &BLACK,
    ))?;

    let mut cc = ChartBuilder::on(&roots[3])
        .margin(10)
        .set_all_label_area_size(50)
        .build_cartesian_2d(trange, erange)?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 20))
        .x_desc("t")
        .y_desc("e")
        .draw()?;

    cc.draw_series(LineSeries::new(
        result.into_iter().map(|(t, (_, _), e)| (t, e)),
        &BLACK,
    ))?;

    println!("#########finished#########");
    Ok(())
}

#[allow(dead_code)]
pub fn plot_energy(
    shots: u32,
    duration: u64,
    initial_state: (f64, f64),
) -> Result<(), Box<dyn std::error::Error>> {
    let root =
        SVGBackend::new("./ode_solve/energy_leapflog3.svg", (4096, 3072)).into_drawing_area();
    root.fill(&WHITE)?;

    let simulator = crate::leap_flog::LeapFlogSimulator::new();
    let (_, _, _, allrange) = simulator.simluate(1.0, duration, initial_state);

    println!("simulating on Leap Flog Simulator");

    let mut cc = ChartBuilder::on(&root)
        .set_all_label_area_size(50)
        .build_cartesian_2d(
            std::ops::Range {
                start: 1.0 / 2_u64.pow(shots) as f64,
                end: 1.0,
            },
            std::ops::Range {
                start: 1000.0,
                end: 10000.0,
            },
        )?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 30))
        .draw()?;

    cc.draw_series(LineSeries::new(
        vec![(1.0, allrange.start + 0.1), (1.0, allrange.end - 0.1)],
        BLACK.stroke_width(3),
    ))?;

    cc.draw_series(PointSeries::of_element(
        vec![(1.0, allrange.start + 0.1), (1.0, allrange.end - 0.1)],
        3,
        &BLACK,
        &|c, s, st| {
            EmptyElement::at(c) + PathElement::new(vec![(-5, 0), (5, 0)], st.stroke_width(s))
        },
    ))?;

    println!("shot 0 done");
    for i in 1..shots {
        let (_, _, _, erange) = simulator.simluate(1.0 / 2_u64.pow(i) as f64, duration, initial_state);

        cc.draw_series(LineSeries::new(
            vec![
                (1.0 / 2_u64.pow(i) as f64, erange.start + 0.1),
                (1.0 / 2_u64.pow(i) as f64, erange.end - 0.1),
            ],
            BLACK.stroke_width(3),
        ))?;

        cc.draw_series(PointSeries::of_element(
            vec![
                (1.0 / 2_u64.pow(i) as f64, erange.start + 0.1),
                (1.0 / 2_u64.pow(i) as f64, erange.end - 0.1),
            ],
            3,
            &BLACK,
            &|c, s, st| {
                EmptyElement::at(c) + PathElement::new(vec![(-5, 0), (5, 0)], st.stroke_width(s))
            },
        ))?;
        println!("shot {} done", i);
    }

    let root2 =
        SVGBackend::new("./ode_solve/energy_symplectic3.svg", (4096, 3072)).into_drawing_area();
    root2.fill(&WHITE)?;

    let simulator2 = crate::symplectic::SymplecticSimulator::new();
    let (_, _, _, allrange2) = simulator2.simluate(1.0, duration, initial_state);

    println!("simulating on Symplectic Simulator");
    println!("{:?}", allrange2);

    let mut cc = ChartBuilder::on(&root2)
        .set_all_label_area_size(50)
        .build_cartesian_2d(
            std::ops::Range {
                start: 1.0 / 2_u64.pow(shots) as f64,
                end: 1.0,
            },
            LogRange(std::ops::Range {
                start: 1000.0,
                end: 10000.0,
            }),
        )?;

    cc.configure_mesh()
        .axis_desc_style(("sans-serif", 30))
        .draw()?;

    cc.draw_series(LineSeries::new(
        vec![(1.0, allrange2.start + 0.1), (1.0, allrange2.end - 0.1)],
        BLACK.stroke_width(3),
    ))?;

    cc.draw_series(PointSeries::of_element(
        vec![(1.0, allrange2.start + 0.1), (1.0, allrange2.end - 0.1)],
        3,
        &BLACK,
        &|c, s, st| {
            EmptyElement::at(c) + PathElement::new(vec![(-5, 0), (5, 0)], st.stroke_width(s))
        },
    ))?;

    println!("shot 0 done");
    for i in 1..shots {
        let simulator2 = crate::symplectic::SymplecticSimulator::new();
        let (_, _, _, erange2) = simulator2.simluate(1.0 / 2_u64.pow(i) as f64, duration, initial_state);

        println!("{:?}", erange2);

        cc.draw_series(LineSeries::new(
            vec![
                (1.0 / 2_u64.pow(i) as f64, erange2.start + 0.1),
                (1.0 / 2_u64.pow(i) as f64, erange2.end - 0.1),
            ],
            BLACK.stroke_width(3),
        ))?;

        cc.draw_series(PointSeries::of_element(
            vec![
                (1.0 / 2_u64.pow(i) as f64, erange2.start + 0.1),
                (1.0 / 2_u64.pow(i) as f64, erange2.end - 0.1),
            ],
            3,
            &BLACK,
            &|c, s, st| {
                EmptyElement::at(c) + PathElement::new(vec![(-5, 0), (5, 0)], st.stroke_width(s))
            },
        ))?;
        println!("shot {} done", i);
    }

    Ok(())
}

pub fn hamiltonian(_t: f64, (q, p): (f64, f64)) -> f64 {
    p * p / 2.0 + q * q / 2.0
}

pub fn hamiltonian_p(_t: f64, (_, p): (f64, f64)) -> f64 {
    p
}

pub fn hamiltonian_q(_t: f64, (q, _): (f64, f64)) -> f64 {
    q
}
