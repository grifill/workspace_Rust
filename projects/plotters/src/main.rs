use eframe::egui;
//use plotters::prelude::*;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "plotters",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(PlottersApp::new(cc))),
    )

    /*
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
    */
}

struct PlottersApp {}

impl PlottersApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        PlottersApp {}
    }
}

impl eframe::App for PlottersApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(r#"Example Plotter"#);
        });
    }
}
