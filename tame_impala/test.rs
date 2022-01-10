extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::loop_once())
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> () {}

fn view(app: &App, _model: &(), frame: Frame) {
    app.main_window().capture_frame("le_parc/images/try_capture.png");
    let draw = app.draw();
    draw.background().color(rgb8(0xff, 0x00, 0x00));
    draw.to_frame(app, &frame).unwrap();
}