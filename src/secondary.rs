use plotters::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub fn plot() -> Result<(), Box<dyn std::error::Error>> {
    let max: u32 = 30;
    let delta = 0.1;

    let xrange = std::ops::Range {
        start: 1.0 / 2_u64.pow(max) as f64,
        end: 1.0,
    };
    let yrange = std::ops::Range {
        start: -((0.3 * std::f64::consts::PI).cos()) - delta,
        end: -((0.3 * std::f64::consts::PI).cos()) + delta,
    };

    let root = SVGBackend::new("./secondary/new.svg", (2048, 1536)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut writer = BufWriter::new(File::create("./secondary/new.txt")?);

    let mut cc = ChartBuilder::on(&root)
        .set_all_label_area_size(50)
        .build_cartesian_2d(LogRange(xrange), yrange)?;

    cc.configure_mesh()
        .x_desc("h")
        .x_label_formatter(&|v| format!("{:e}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(
        vec![
            (
                1.0 / 2_u64.pow(max) as f64,
                -((0.3 * std::f64::consts::PI).cos()),
            ),
            (1.0, -((0.3 * std::f64::consts::PI).cos())),
        ],
        BLACK.stroke_width(5),
    ))?;

    cc.draw_series(LineSeries::new(
        (0..1000)
            .map(|v| 1.0 / (max as f64 * v as f64 / 1000.0).exp2())
            .map(|v| {
                (
                    v,
                    -((0.3 * std::f64::consts::PI).cos())
                        + (0.3 * std::f64::consts::PI).cos() * v * v / 12.0,
                )
            }),
        GREEN.stroke_width(5),
    ))?;

    writer.write_all(String::from("1st\n").as_bytes())?;

    cc.draw_series(PointSeries::of_element(
        (0_u32..max).map(|v| {
            let h = 1.0 / 2_u64.pow(v) as f64;
            let string = format!(
                "dx:{},\tcos(x + dx):{},\tcos(x):{},\tcos(x - dx):{},\td^2y/dx^2:{}\n",
                h,
                (0.3 * std::f64::consts::PI + h).cos(),
                (0.3 * std::f64::consts::PI).cos(),
                (0.3 * std::f64::consts::PI - h).cos(),
                ((0.3 * std::f64::consts::PI + h).cos() * 2_u64.pow(v) as f64
                    + (0.3 * std::f64::consts::PI - h).cos() * 2_u64.pow(v) as f64
                    - 2.0 * (0.3 * std::f64::consts::PI).cos() * 2_u64.pow(v) as f64)
                    * 2_u64.pow(v) as f64
            );
            writer.write_all(string.as_bytes()).unwrap();
            (
                h,
                ((0.3 * std::f64::consts::PI + h).cos() * 2_u64.pow(v) as f64
                    + (0.3 * std::f64::consts::PI - h).cos() * 2_u64.pow(v) as f64
                    - 2.0 * (0.3 * std::f64::consts::PI).cos() * 2_u64.pow(v) as f64)
                    * 2_u64.pow(v) as f64,
            )
        }),
        6,
        &RED,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))
    .unwrap()
    .label("(f(x + h) - 2 f(x) + f(x - h)) / h^2")
    .legend(|(x, y)| Circle::new((x, y), 3, RED.filled()));

    writer.flush()?;

    cc.configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}
