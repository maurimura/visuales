mod config;
use config::{ColorRgb, PALETTE};
use nannou::{prelude::*, winit::event::DeviceEvent};
use utils::gif_creator;

const WORKSPACE: &str = "le_parc";
const TITLE: &str = "waves";
const POINTS: u16 = 1024;
const BACKGROUND_COLOR: &ColorRgb = &ColorRgb(0, 0, 0);
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
    let draw = app.draw();
    let boundary = app.window_rect();
    draw.background().color(BACKGROUND_COLOR);
    let amount_of_colors = PALETTE.len();

    let width = boundary.right() + 20.0;
    let height = boundary.top() * 2.0;
    let stroke_weight = height / 16.0;
    let phase = phase(app);

    let points_odd: Vec<Vec2> = (0..POINTS)
        .map(|i| {
            let px = map_range(i as f32, 0.0, POINTS as f32, -width, width);
            let x = map_range(i as f32, 0.0, POINTS as f32, 0.0, 10.0 * 2.0 * PI);
            let py = (x + phase).sin() * 5.0;
            pt2(px, py)
        })
        .collect();

    let points_even: Vec<Vec2> = (0..POINTS)
        .map(|i| {
            let px = map_range(i as f32, 0.0, POINTS as f32, -width, width);
            let x = map_range(i as f32, 0.0, POINTS as f32, 0.0, 9.0 * 2.0 * PI);
            let py = (x - phase).sin() * 5.0;
            pt2(px, py)
        })
        .collect();

    for index in 0..amount_of_colors + 1 {
        let color = match PALETTE.get(index) {
            Some(color) => *color,
            None => &BACKGROUND_COLOR,
        };
        let points = if index % 2 == 0 {
            points_even.clone()
        } else {
            points_odd.clone()
        };

        let points_colored: Vec<_> = points
            .iter()
            .map(|point| {
                (
                    pt2(
                        point.x,
                        point.y
                            + ((amount_of_colors / 2) as f32 - index as f32) * stroke_weight / 2.0,
                    ),
                    color,
                )
            })
            .collect();

        draw.polyline()
            .weight(stroke_weight)
            .points_colored(points_colored);
    }

    if phase < TAU {
        gif_creator::save_frame(app, model._window, WORKSPACE, TITLE);
    } 

    draw.to_frame(app, &frame).unwrap();
}

fn phase(app: &App) -> f32 {
    map_range(app.elapsed_frames() as f32, 0.0, 90.0, 0.0, TAU)
}
