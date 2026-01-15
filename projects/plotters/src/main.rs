use plotters::prelude::*;

fn main() {
  let root_area = BitMapBackend::new("./images/LinePlotDemo.png", (640, 480))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Line Plot Demo", ("sans-serif", 40))
    .build_cartesian_2d(-25..25, 0..250)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  ctx.draw_series(
    LineSeries::new((-50..=50).map(|x| (x, x* 2*x)), &BLUE)
  ).unwrap();
}