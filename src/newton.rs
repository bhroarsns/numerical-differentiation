use plotters::prelude::*;

#[allow(dead_code)]
pub fn find_root(init: f64) -> Result<(), Box<dyn std::error::Error>> {
    let delta = 0.0001;
    let epsilon = 0.0001;

    let mut vmin: f64 = testing(init);
    let mut vmax: f64 = testing(init);
    let mut ymin: f64 = init;
    let mut ymax: f64 = init;

    let mut candidate = init;
    let mut count = 0;
    let mut result: Vec<(u64, f64)> = vec![(count, candidate)];
    let mut data: Vec<((f64, f64), u64)> = vec![((candidate, testing(candidate)), count)];

    loop {
        count += 1;
        {
            let error = -testing(candidate) / derivative(candidate);
            candidate += error;

            if vmin > testing(candidate) {
                vmin = testing(candidate)
            }

            if vmax < testing(candidate) {
                vmax = testing(candidate)
            }

            if ymax < candidate {
                ymax = candidate;
            }

            if ymin > candidate {
                ymin = candidate;
            }

            if error.abs() < epsilon {
                result.push((count, candidate));
                data.push(((candidate, testing(candidate)), count));
                println!("finished by epsilon");
                break;
            }
        }
        result.push((count, candidate));
        data.push(((candidate, testing(candidate)), count));

        if testing(candidate).abs() < delta {
            println!("finished by delta");
            break;
        }

        if count == u8::MAX as u64 {
            println!("too long. interrupting...");
            break;
        }
    }

    ymin -= 0.1;
    ymax += 0.1;
    vmin -= 0.1;
    vmax += 0.1;

    let filename = format!("./newton/{}.svg", init);
    let root = SVGBackend::new(&filename, (4096, 3072)).into_drawing_area();
    let (up, down) = root.split_vertically(2048);
    up.fill(&WHITE)?;
    down.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&up)
        .set_all_label_area_size(50)
        .caption(
            format!(
                "estimated answer is {} with epsilon = {}, delta = {}, in {} steps",
                candidate, epsilon, delta, count
            ),
            ("Sans-Serif", 30),
        )
        .build_cartesian_2d(-1..(count as i32 + 1), ymin..ymax)?;

    cc.configure_mesh().draw()?;

    cc.draw_series(LineSeries::new(
        vec![
            (-1, -0.254_461_295_051_336_86),
            ((count + 1) as i32, -0.254_461_295_051_336_86),
        ],
        RGBColor(160, 160, 160).stroke_width(5),
    ))?;

    cc.draw_series(PointSeries::of_element(
        result,
        10,
        &BLACK,
        &|(x, y), s, st| EmptyElement::at((x as i32, y)) + Circle::new((0, 0), s, st.filled()),
    ))?;

    let mut cc = ChartBuilder::on(&down)
        .set_all_label_area_size(50)
        .build_cartesian_2d(ymin..ymax, vmin..vmax)?;

    cc.configure_mesh().draw()?;

    cc.draw_series(LineSeries::new(
        vec![
            (-0.254_461_295_051_336_86, vmin),
            (-0.254_461_295_051_336_86, vmax),
        ],
        RGBColor(160, 160, 160).stroke_width(5),
    ))?;

    cc.draw_series(LineSeries::new(
        (0..1001)
            .map(|v| ymin + (ymax - ymin) * v as f64 / 100.0)
            .map(|v| (v, testing(v))),
        RED.stroke_width(5),
    ))?;

    cc.draw_series(PointSeries::of_element(
        data,
        10,
        &BLACK,
        &|(c, i), s, st| {
            EmptyElement::at(c)
                + Circle::new((0, 0), s, st.filled())
                + Text::new(
                    format!("{}", i),
                    (-30_i32, (i as i32 % 3 - 1) * 30),
                    ("Sans-Serif", 30),
                )
        },
    ))?;

    Ok(())
}

fn testing(x: f64) -> f64 {
    x.tanh() + 0.2 * x + 0.3
}

fn derivative(x: f64) -> f64 {
    1.0 / (x.cosh() * x.cosh()) + 0.2
}
