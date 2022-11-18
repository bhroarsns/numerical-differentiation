use plotters::prelude::*;

#[allow(dead_code)]
pub fn find_root(init: f64) -> Result<(), Box<dyn std::error::Error>> {
    let step = 10.0;
    let epsilon = 0.0001;

    let mut range: (f64, f64) = (init, init);

    println!("look for initial state...");

    if is_near_enough_up(init, epsilon) {
        range = (init, init + epsilon);
    } else if is_near_enough_down(init, epsilon) {
        range = (init - epsilon, init);
    }else {
        let mut big = init + step;
        let mut small = init - step;

        for _ in 0..u8::MAX {
            if is_near_enough_up(big, epsilon) {
                range = (big, big + epsilon);
                break;
            } else if is_near_enough_down(big, epsilon) {
                range = (big - epsilon, big);
                break;
            }else if testing(init) * testing(big) < 0.0 {
                range = (init, big);
                break;
            } else {
                big += step;
            }
            
            if is_near_enough_up(small, epsilon) {
                range = (small, small + epsilon);
                break;
            } else if is_near_enough_down(small, epsilon) {
                range = (small - epsilon, small);
                break;
            }else if testing(init) * testing(small) < 0.0 {
                range = (small, init);
                break;
            } else {
                small -= step;
            }
        }
    }

    println!("initital state found : {:?}", range);

    let mut data: Vec<(u64, (f64, f64))> = Vec::new();
    data.push((0, range));
    let initial = (range.0 - 0.1, range.1 + 0.1);

    let mut count = 0;
    loop {
        if range.1 - range.0 < epsilon {
            break;
        } else {
            let middle = (range.0 + range.1) / 2.0;
            if is_near_enough_up(middle, epsilon) && range.1 - range.0 >= 2.0 * epsilon {
                range = (middle, middle + epsilon);
                data.push((count + 1, range));
                count += 1;
                break;
            } else if is_near_enough_down(middle, epsilon) && range.1 - range.0 >= 2.0 * epsilon {
                range = (middle - epsilon, middle);
                data.push((count + 1, range));
                count += 1;
                break;
            } else if testing(middle) * testing(range.1) < 0.0 {
                range = (middle, range.1);
                data.push((count + 1, range));
            } else {
                if testing(middle) * testing(range.0) >= 0.0 {
                    panic!("something went wrong")
                }
                range = (range.0, middle);
                data.push((count + 1, range));
            }
        }
        count += 1
    }

    let filename = format!("./bisection/{}.svg", init);

    let root = SVGBackend::new(&filename, (4096, 3072)).into_drawing_area();
    let (up, down) = root.split_vertically(2048);
    up.fill(&WHITE)?;
    down.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&up)
        .caption(format!("answer found between {} and {} in {} steps", range.0, range.1, count), ("Sansd-Serif", 30))
        .set_all_label_area_size(50)
        .build_cartesian_2d(-1_f64..(count + 1) as f64, initial.0..initial.1)?;

    cc.configure_mesh()
        .draw()?;

    cc.draw_series(LineSeries::new(
        vec![(-1.0, -0.25446129505133685446563430575662), ((count + 1) as f64, -0.25446129505133685446563430575662)],
        RGBColor(160, 160, 160).stroke_width(5)
    ))?;
    
    cc.draw_series(PointSeries::of_element(
        data.clone(),
        10,
        &BLACK,
        &|(i, (b, t)), s, st| {
            return PathElement::new(vec![(i as f64, b), (i as f64, t)], st.stroke_width(s))
        }
    ))?;

    cc.draw_series(PointSeries::of_element(
        data.clone(),
        5,
        &BLACK,
        &|(i, (b, _)), s, st| {
            return PathElement::new(vec![(i as f64 - 0.1, b), (i as f64 + 0.1, b)], st.stroke_width(s))
        }
    ))?;

    cc.draw_series(PointSeries::of_element(
        data.clone(),
        5,
        &BLACK,
        &|(i, (_, t)), s, st| {
            return PathElement::new(vec![(i as f64 - 0.1, t), (i as f64 + 0.1, t)], st.stroke_width(s))
        }
    ))?;

    let mut cc2 = ChartBuilder::on(&down)
        .set_all_label_area_size(50)
        .build_cartesian_2d(initial.0..initial.1, testing(initial.0)..testing(initial.1))?;
    
    cc2.configure_mesh()
        .draw()?;

    cc2.draw_series(LineSeries::new(
        vec![(-0.25446129505133685446563430575662, testing(initial.0)), (-0.25446129505133685446563430575662, testing(initial.1))],
        RGBColor(160, 160, 160).stroke_width(5)
    ))?;
    
    cc2.draw_series(LineSeries::new(
        (0..1001).map(|v| initial.0 + (initial.1 - initial.0) * v as f64 / 100.0).map(|v| {
            (v, testing(v))
        }),
        RED.stroke_width(5)
    ))?;

    cc2.draw_series(PointSeries::of_element(
        data.clone(),
        10,
        &BLACK,
        &|(_, (b, _)), s, st| {
            return EmptyElement::at((b, testing(b)))
            + Circle::new((0, 0), s, st.filled())
        }))?;

    cc2.draw_series(PointSeries::of_element(
        data,
        10,
        &BLACK,
        &|(_, (_, t)), s, st| {
            return EmptyElement::at((t, testing(t)))
            + Circle::new((0, 0), s, st.filled())
        }))?;
    
    Ok(())
}

fn testing(x: f64) -> f64 {
    x.tanh() + 0.2 * x + 0.3
}

fn is_near_enough_up(x: f64, e: f64) -> bool {
    testing(x + e) * testing(x) < 0.0
}

fn is_near_enough_down(x: f64, e:f64) -> bool {
    testing(x + e) * testing(x) < 0.0
}