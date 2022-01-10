mod config;

use nannou::{geom::Range, prelude::*, winit::event::DeviceEvent};
use utils::gif_creator;

use config::PALETTE;
const TITLE: &str = "squares";
const WORKSPACE: &str = "le_parc";

const SIDE: i32 = 28;
const ANIMATION_PERIOD: f32 = 4.0;

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
    if app.elapsed_frames() as f32 % ANIMATION_PERIOD == 0.0 {
        let boundary = app.window_rect();
        let draw = app.draw();

        draw.background().color(BLACK);

        let canvas_width = boundary.right() * 2.0;
        let canvas_height = boundary.top() * 2.0;

        let w = canvas_width / SIDE as f32;
        let h = canvas_height / SIDE as f32;

        let color_offset = (app.elapsed_frames() as f32 / ANIMATION_PERIOD).floor() as i32;

        for x in 0..SIDE {
            for y in 0..SIDE {
                let px = map_range(x, 0, SIDE, boundary.left() + w, boundary.right() - w) + w * 0.5;
                let py = map_range(y, 0, SIDE, boundary.top() - w, boundary.bottom() + w) - h * 0.5;

                let amount_of_colors = PALETTE.len() as i32;
                let color_square_index = ((x + y + color_offset) % amount_of_colors) as usize;
                let color_dot_index =
                    (((-x + y + color_offset - 1) % -amount_of_colors) as i32).abs() as usize;


                let color_square = PALETTE[color_square_index];
                let color_dot = PALETTE[color_dot_index];

                let rect_boundary = Rect::<f32> {
                    x: Range::new(px - w, px + w),
                    y: Range::new(py - h, py + h),
                };

                draw.rect().w_h(w, h).x_y(px, py).color(color_square);

                draw.ellipse()
                    .w_h(rect_boundary.w() * 0.5 * 0.8, rect_boundary.h() * 0.5 * 0.8)
                    .x_y(rect_boundary.x() - 0.8, rect_boundary.y() + 0.7)
                    .color(color_dot);
            }
        }

        if (app.elapsed_frames() as f32 / ANIMATION_PERIOD) < 28.0 {
            gif_creator::save_frame(app, model._window, WORKSPACE, TITLE);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
