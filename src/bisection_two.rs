use plotters::prelude::*;

#[allow(dead_code)]
pub fn find_root(init: f64) -> Result<(), Box<dyn std::error::Error>> {
    let mut step = 1.0;
    let epsilon = 0.0001;
    let delta = 0.0001;

    let mut range: Option<(f64, f64, bool)> = None;
    let mut direct_answer: Option<(u64, f64)> = None;
    let mut data: Vec<(u64, (f64, f64))> = Vec::new();
    let mut count = 0;
    let caption_text;
    let initial;

    println!("look for initial state...");

    if testing(init).abs() < delta {
        direct_answer = Some((count, init));
    } else {
        let sign = testing(init).is_sign_positive();
        let mut big = init + step;
        let mut small = init - step;

        'define_step: for _ in 0..u8::MAX {
            for _ in 0..u8::MAX {
                let b = testing(big);
                if b.abs() < delta {
                    direct_answer = Some((count, big));
                    break 'define_step;
                } else if sign ^ b.is_sign_positive() {
                    range = Some((big - step, big, !sign));
                    break 'define_step;
                } else {
                    big += step;
                }
    
                let s = testing(small);
                if s.abs() < delta {
                    direct_answer = Some((count, small));
                    break 'define_step;
                } else if sign ^ s.is_sign_positive() {
                    range = Some((small, small + step, sign));
                    break 'define_step;
                } else {
                    small -= step;
                }
            }
            step *= 129.0 / 256.0;
        }
    }

    match range {
        Some((s, e, increase)) => {
            initial = (s - 0.1, e + 0.1);

            let mut current_range = (s, e);
            println!("initital state found : {:?}", (s, e));
            data.push((count, current_range));

            if current_range.1 - current_range.0 >= epsilon {
                loop {
                    count += 1;

                    let middle = (current_range.0 + current_range.1) / 2.0;
                    let m = testing(middle);

                    if m.abs() < delta {
                        direct_answer = Some((count, middle));
                        break;
                    } else if m.is_sign_positive() ^ increase {
                        current_range = (middle, current_range.1);
                        data.push((count, current_range));
                    } else {
                        current_range = (current_range.0, middle);
                        data.push((count, current_range));
                    }

                    if current_range.1 - current_range.0 < epsilon {
                        break;
                    }
                }
            }
        }
        None => {
            initial = (direct_answer.unwrap().1 - 2.0 * delta, direct_answer.unwrap().1 + 2.0 * delta);
        }
    }

    caption_text = match direct_answer {
        Some(i) => format!("estimated answer: {} found in {} steps", i.1, i.0),
        None => format!(
            "answer found betweeen {} and {} in {} steps",
            &data.last().unwrap().1 .0,
            &data.last().unwrap().1 .1,
            &data.last().unwrap().0
        ),
    };

    let filename = format!("./bisection2/{}.svg", init);
    let root = SVGBackend::new(&filename, (4096, 3072)).into_drawing_area();
    let (up, down) = root.split_vertically(2048);
    up.fill(&WHITE)?;
    down.fill(&WHITE)?;

    //共通部分
    let mut cc = ChartBuilder::on(&up)
        .caption(&caption_text, ("Sans-Serif", 30))
        .set_all_label_area_size(50)
        .build_cartesian_2d(-1.0..(count + 1) as f64, initial.0..initial.1)?;

    cc.configure_mesh().draw()?;

    cc.draw_series(LineSeries::new(
        vec![
            (-1.0, -0.254_461_295_051_336_86),
            ((count + 1) as f64, -0.254_461_295_051_336_86),
        ],
        RGBColor(192, 192, 192).stroke_width(5),
    ))?;

    let mut cc2 = ChartBuilder::on(&down)
        .set_all_label_area_size(50)
        .build_cartesian_2d(initial.0..initial.1, testing(initial.0)..testing(initial.1))?;

    cc2.configure_mesh().draw()?;

    cc2.draw_series(LineSeries::new(
        vec![
            (-0.254_461_295_051_336_86, testing(initial.0)),
            (-0.254_461_295_051_336_86, testing(initial.1)),
        ],
        RGBColor(192, 192, 192).stroke_width(5),
    ))?;

    cc2.draw_series(LineSeries::new(
        (0..1001)
            .map(|v| initial.0 + (initial.1 - initial.0) * v as f64 / 100.0)
            .map(|v| (v, testing(v))),
        RED.stroke_width(5),
    ))?;

    //dataが空でない時
    if range.is_some() {
        cc.draw_series(PointSeries::of_element(
            data.clone(),
            10,
            &BLACK,
            &|(i, (b, t)), s, st| {
                PathElement::new(vec![(i as f64, b), (i as f64, t)], st.stroke_width(s))
            },
        ))?;

        cc.draw_series(PointSeries::of_element(
            data.clone(),
            5,
            &BLACK,
            &|(i, (b, _)), s, st| {
                PathElement::new(
                    vec![(i as f64 - 0.1, b), (i as f64 + 0.1, b)],
                    st.stroke_width(s),
                )
            },
        ))?;

        cc.draw_series(PointSeries::of_element(
            data.clone(),
            5,
            &BLACK,
            &|(i, (_, t)), s, st| {
                PathElement::new(
                    vec![(i as f64 - 0.1, t), (i as f64 + 0.1, t)],
                    st.stroke_width(s),
                )
            },
        ))?;

        cc2.draw_series(PointSeries::of_element(
            data.clone(),
            10,
            &BLACK,
            &|(_, (b, _)), s, st| {
                EmptyElement::at((b, testing(b))) + Circle::new((0, 0), s, st.filled())
            },
        ))?;

        cc2.draw_series(PointSeries::of_element(
            data,
            10,
            &BLACK,
            &|(_, (_, t)), s, st| {
                EmptyElement::at((t, testing(t))) + Circle::new((0, 0), s, st.filled())
            },
        ))?;
    }

    //最後答えが確定した時
    if let Some((i, x)) = direct_answer {
        cc.draw_series(PointSeries::of_element(
            vec![(i as f64, x)],
            10,
            &BLACK,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;

        cc2.draw_series(PointSeries::of_element(
            vec![(x, testing(x))],
            10,
            &GREEN,
            &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
        ))?;
    }

    Ok(())
}

fn testing(x: f64) -> f64 {
    x.tanh() + 0.2 * x + 0.3
}
