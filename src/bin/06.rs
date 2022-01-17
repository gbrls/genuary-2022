// Color gradients gone wrong.

use genuary_2022::*;
use nannou::prelude::*;

const W: u32 = 920;
const H: u32 = 920;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn event_update(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseWheel(delta, _) => match delta {
            MouseScrollDelta::LineDelta(_, v) => {}
            _ => {}
        },
        KeyPressed(Key::Return) => app.save_animation(),
        _ => {}
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .event(event_update)
        .view(view)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::loop_once());
    app.main_window().set_inner_size_pixels(W, H);

    Model { _window }
}

fn update(app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.polyline()
        .stroke_weight(1.0)
        .color(WHITE)
        .points((0..800).map(|x| vec2(x as f32, (x as f32 / 30.0).sin() * 100.0)));

    draw.to_frame(app, &frame).unwrap();
}
