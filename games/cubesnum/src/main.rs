use miniquad::conf;

fn main() {
    let conf = conf::Conf {
        window_title: "Rust Cubes Demo".to_string(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    };
    //miniquad::start(conf, /* f */);
}
