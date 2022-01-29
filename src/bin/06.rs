// Color gradients gone wrong.

use genuary_2022::*;
use nannou::{
    image::{DynamicImage, GenericImageView},
    prelude::*,
};

const W: u32 = 1500;
const H: u32 = 800;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    image: DynamicImage,
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
    //let img_path = app.assets_path().unwrap().join("image-00.png");
    let img_path = app.assets_path().unwrap().join("berserk-00.jpg");

    let image = nannou::image::io::Reader::open(&img_path)
        .unwrap()
        .decode()
        .unwrap();

    let _window = app
        .new_window()
        .event(event_update)
        .view(view)
        .build()
        .unwrap();

    //app.set_loop_mode(LoopMode::loop_once());
    app.main_window().set_inner_size_pixels(W, H);

    Model { _window, image }
}

fn update(app: &App, _model: &mut Model, _update: Update) {}

fn brightness(r: f32, g: f32, b: f32) -> f32 {
    vec3(r, g, b).length()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);

    for _ in 0..1_00 {
        let b = 200.0;
        let x = random_range(-b, b);
        let y = random_range(-b, b);

        let r = random_range(0.7, 0.89);
        let g = random_range(0.6, 0.72);
        let b = random_range(0.4, 0.52);

        draw.ellipse().w_h(2.0, 2.0).x_y(x, y).color(rgb(r, g, b));
    }

    draw.to_frame(app, &frame).unwrap();
}
