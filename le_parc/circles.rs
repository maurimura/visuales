mod config;
use nannou::{prelude::*, winit::event::DeviceEvent};

use config::{PALETTE, ColorRgb};

// use super::config::{PALETTE, ColorRgb};
pub struct Model {
    _window: window::Id,
    palette: Vec<&'static ColorRgb>,
    backwards: bool,
}
fn main() {
    nannou::app(model).event(event).update(update).run();
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let vec_palette: Vec<&ColorRgb> = PALETTE.to_vec();
    Model {
        _window,
        palette: vec_palette,
        backwards: false,
    }
}

pub fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::DeviceEvent(_device, device_event) = event {
        match device_event {
            DeviceEvent::Key(key) => {
                model.backwards = key.virtual_keycode == Some(Key::Left);
            }
            _ => {}
        }
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 6 == 0 {
        let mut colors = model.palette.clone();

        if !model.backwards {
            let mut new_colors = colors.split_off(1);
            new_colors.push(colors[0]);
            model.palette = new_colors;
        } else {
            if let Some(new_first_color) = colors.pop() {
                let mut new_colors = vec![new_first_color];
                new_colors.extend_from_slice(&colors[0..]);
                model.palette = new_colors;
            }
        }
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(BLACK);
    let amount_of_colors = model.palette.len();
    let radius = boundary.top() / amount_of_colors as f32;

    for index in 0..amount_of_colors {
        let current_radius = (amount_of_colors - index) as f32 * radius;
        draw.ellipse()
            .w_h(current_radius, current_radius)
            .color(model.palette[index]);
    }

    draw.to_frame(app, &frame).unwrap();
}
