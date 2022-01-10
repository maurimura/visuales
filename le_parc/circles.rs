mod config;
use nannou::{prelude::*, winit::event::DeviceEvent};

use config::PALETTE;
use utils::gif_creator;

const WORKSPACE: &str = "le_parc";
const TITLE: &str = "circles";
pub struct Model {
    _window: window::Id,
}
fn main() {
    nannou::app(model).event(event).update(update).run();
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    Model { _window }
}

pub fn event(_app: &App, _model: &mut Model, event: Event) {
    if let Event::DeviceEvent(_device, device_event) = event {
        match device_event {
            DeviceEvent::Key(key) => {
                if key.virtual_keycode == Some(Key::S) {
                    gif_creator::create_gif(WORKSPACE, TITLE);
                }
            }
            _ => {}
        }
    }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let frame_delay = 6;
    if app.elapsed_frames() % frame_delay == 0 {
        let draw = app.draw();
        let boundary = app.window_rect();
        draw.background().color(BLACK);
        let amount_of_colors = PALETTE.len();
        let max_radius = boundary.top().min(boundary.right()) + 1.0;
        let stroke_weight = max_radius / amount_of_colors as f32;
        let points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x = radian.cos();
            let y = radian.sin();
            pt2(x, y)
        });

        for index in 0..amount_of_colors {
            let amount_of_colors = amount_of_colors as u64;
            let radius = map_range(index as f32, 0.0, 15.0, 10.0, max_radius);
            let span = app.elapsed_frames() / frame_delay;
            let color_index = (index as u64 - 16 - span) % amount_of_colors as u64;

            let colored_points = points.clone().map(|point| {
                (
                    pt2(point.x * radius, point.y * radius),
                    PALETTE[color_index as usize],
                )
            });

            draw.polyline()
                .weight(stroke_weight)
                .points_colored(colored_points);
        }

        if app.elapsed_frames() / frame_delay < 14 {
            gif_creator::save_frame(app, model._window, WORKSPACE, TITLE);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}