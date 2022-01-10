use nannou::{prelude::*, winit::event::DeviceEvent};
use utils::gif_creator;
pub struct Model {
    _window: window::Id,
}

const WORKSPACE: &str = "tame_impala";
const TITLE: &str = "circles";
fn main() {
    nannou::app(model).event(event).update(update).run();
}

pub fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
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
    let frames = app.elapsed_frames();
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(BLACK);
    let radius = boundary.top().min(boundary.right()) / 2.0;
    let velocity = frames as f32 * 0.1;

    let points = (0..=360).map(|i| {
        let frames = deg_to_rad((frames) as f32) * (8.0 * TAU).min(velocity);
        let px = frames.cos() * radius;
        let py = frames.sin() * radius;
        let radian = deg_to_rad(i as f32);
        let x = radian.cos() * radius;
        let y = radian.sin() * radius;
        pt2(x + px, y + py)
    });

    let transformed_points = points.map(|point| (point, AQUAMARINE));

    if velocity < 10.0 * TAU {
        gif_creator::save_frame(app, model._window, WORKSPACE, TITLE);
    }

    draw.polyline()
        .weight(10.0)
        .points_colored(transformed_points);

    draw.to_frame(app, &frame).unwrap();
}
