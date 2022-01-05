use genuary_2022::capture_path;
/// Draw 10,000 of something.
use nannou::prelude::*;

const N: u32 = 10_000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    app.set_loop_mode(LoopMode::loop_once());
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let v_x = 350.0f32;
    let v_r = 80.0f32;

    for _ in 0..N {
        draw.ellipse()
            .radius(random_range(v_r/2.0, v_r))
            .stroke_weight(2.0)
            .stroke_color(rgba(0.8, 0.8, 0.8, 0.008))
            .color(rgba(0.1, 0.2, 0.3, 0.00))
            .x_y(random_range(-v_x, v_x), random_range(-v_x, v_x));
    }

    app.main_window().capture_frame(capture_path(app));
    draw.to_frame(app, &frame).unwrap();
}
