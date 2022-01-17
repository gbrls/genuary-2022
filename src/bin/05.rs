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

    //app.set_loop_mode(LoopMode::loop_once());
    app.main_window().set_inner_size_pixels(W, H);

    Model { _window }
}

fn update(app: &App, _model: &mut Model, _update: Update) {}

fn random_col() -> (f32, f32, f32) {
    (
        random_range(0.0, 1.0),
        random_range(0.5, 0.95),
        random_range(0.1, 0.8),
    )
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let a = random_col();
    let b = random_col();

    let w = 38.0;
    let w_shrink = 0.5;

    let mut vec = Vec::new();

    for _ in 0..250 {
        let r = (
            random_range(-300.0f32, 300.0f32),
            random_range(-300.0, 80.0f32),
        );
        vec.push(r)
    }

    vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for start in vec {
        let d = random_range(100.0, 350.0);

        let n = 20;
        let rot = random_range(0.5, 3.8);
        let ang = (random_range(-0.5, 0.5), random_range(-0.5, 0.5));
        let alpha = 1.00;

        for i in 0..=n {
            let u = i as f32 / n as f32;
            let v = 1.0 - u;

            let c = (a.0 * u + b.0 * v, a.1 * u + b.1 * v, a.2 * u + b.2 * v);

            draw.rect()
                .rotate(rot)
                .w_h(
                    (1.0 - w_shrink) * w + w_shrink * v,
                    (1.0 - w_shrink) * w + w_shrink * v,
                )
                .x_y(start.0 + (d * u) * ang.0, start.1 + d * u * ang.1)
                .color(hsla(c.0, c.1, c.2, alpha))
                .stroke_weight(1.0)
                .stroke_color(hsla(0.0, 0.0, u.log2(), alpha));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
