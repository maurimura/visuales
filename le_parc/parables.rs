mod config;

use config::{ColorRgb, PALETTE};
use nannou::{geom::Range, prelude::*, winit::event::DeviceEvent};
use utils::gif_creator;

const WORKSPACE: &str = "le_parc";
const TITLE: &str = "parables";
const POINTS: u16 = 2000;

pub struct Model {
    _window: window::Id,
    palette: Vec<&'static ColorRgb>,
}

fn main() {
    nannou::app(model).event(event).update(update).run();
}

pub fn model(app: &App) -> Model {
    let vec_palette: Vec<&ColorRgb> = PALETTE.to_vec();
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    Model {
        _window,
        palette: vec_palette,
    }
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
    let height = boundary.top();
    let width = boundary.right();
    let size = height.min(width);
    let fixed_boundary = Rect {
        x: Range::new(-size, size),
        y: Range::new(-size, size),
    };
    draw.background().color(BLACK);

    draw_side_parable(&draw, fixed_boundary, model);
    draw_lower_parable(&draw, fixed_boundary, model);

    if app.elapsed_frames() < 5 {
        gif_creator::save_frame(app, model._window, WORKSPACE, TITLE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_side_parable(draw: &Draw, boundary: Rect, model: &Model) {
    let width = boundary.top();
    let height = boundary.right() / 2.0;

    let span = width / 9.0;

    for i in 1..10 {
        let color = model.palette[i - 1];
        let y = i as f32 * span;
        let x = parable(y, height, width);

        // Draw dots in the outter rectangle
        for _j in 0..POINTS {
            let py = map_range(
                rand_point_growing_probability(),
                0.0,
                1.0,
                (i as f32 - 1.0) * span,
                y,
            );
            let px = map_range(random::<f32>(), 0.0, 1.0, x, boundary.left());

            if px < parable(py, height, width) {
                draw.ellipse().w_h(2.0, 2.0).x_y(px, py).color(color);
                draw.ellipse().w_h(2.0, 2.0).x_y(-px, py).color(color);
            }
        }

        // Draw dots in the inner triangle
        for _j in 0..POINTS * 2 {
            // First, draw random dots in a rectangle with:
            // width: upper-radius
            // height: the height of the triangle

            let upper_radius_point_y = (i - 1) as f32 * span;
            let upper_radius_point_x = parable(upper_radius_point_y, height, width);
            let upper_radius_point = pt2(upper_radius_point_x, upper_radius_point_y);
            let bottom_radius_point = pt2(x, y);

            let width = bottom_radius_point.length();
            let projected_vertex = upper_radius_point.project_onto(bottom_radius_point);
            let height = projected_vertex.distance(upper_radius_point);

            let randx = random::<f32>();
            let randy = rand_point_growing_probability();

            if randy < randx + 0.1 {
                let px = map_range(randx, 0.0, 1.0, 0.0, width);
                let py = map_range(randy, 0.0, 1.0, 0.0, -height);

                let angle = upper_radius_point.angle();
                let p = pt2(px, py).rotate(angle);

                draw.ellipse().w_h(2.0, 2.0).x_y(p.x, p.y).color(color);
                draw.ellipse().w_h(2.0, 2.0).x_y(-p.x, p.y).color(color);
            }
        }
    }
}

fn draw_lower_parable(draw: &Draw, boundary: Rect, model: &Model) {
    let height = boundary.top() / 2.0;
    let width = boundary.right();

    let span = width / 9.0;

    for i in 1..10 {
        let color = model.palette[i - 1];
        let x = i as f32 * span;
        let y = parable(x, height, width);

        for _j in 0..POINTS {
            let px = map_range(
                rand_point_growing_probability(),
                0.0,
                1.0,
                (i as f32 - 1.0) * span,
                x,
            );
            let py = map_range(random::<f32>(), 0.0, 1.0, boundary.bottom(), y);

            if py < parable(px, height, width) {
                draw.ellipse().w_h(2.0, 2.0).x_y(px, py).color(color);
                draw.ellipse().w_h(2.0, 2.0).x_y(-px, py).color(color);
            }
        }
        // Draw dots in the inner triangle
        for _j in 0..POINTS * 2 {
            // First, draw random dots in a rectangle with:
            // width: upper-radius
            // height: the height of the triangle

            let bottom_radius_point_x = (i - 1) as f32 * span;
            let bottom_radius_point_y = parable(bottom_radius_point_x, height, width);
            let bottom_radius_point = pt2(bottom_radius_point_x, bottom_radius_point_y);
            let upper_radius_point = pt2(x, y);

            let projected_vertex = bottom_radius_point.project_onto(upper_radius_point);
            let height = projected_vertex.distance(bottom_radius_point);
            let width = upper_radius_point.length();

            let randx = random::<f32>();
            let randy = rand_point_growing_probability();

            if randy < randx + 0.1 {
                let px = map_range(randx, 0.0, 1.0, 0.0, width);
                let py = map_range(randy, 0.0, 1.0, 0.0, height);

                let angle = bottom_radius_point.angle();
                let p = pt2(px, py).rotate(angle);

                draw.ellipse().w_h(2.0, 2.0).x_y(p.x, p.y).color(color);
                draw.ellipse().w_h(2.0, 2.0).x_y(-p.x, p.y).color(color);
            }
        }
    }
}

fn fake_probability_distribution(x: f32) -> f32 {
    x * x + 0.1
}

fn rand_point_growing_probability() -> f32 {
    loop {
        let rand1 = random::<f32>();
        let rand2 = random::<f32>();

        if rand1 < fake_probability_distribution(rand2) {
            return rand2;
        }
    }
}

fn parable(x: f32, h: f32, w: f32) -> f32 {
    (x * x * h) / (w * w) - h
}
