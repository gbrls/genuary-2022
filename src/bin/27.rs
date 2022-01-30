// Palette: #2E294E #541388 #F1E9DA #FFD400 #D90368

use genuary_2022::*;
use nannou::{
    color::{encoding::Srgb, IntoColor},
    image::{DynamicImage, GenericImageView},
    prelude::*,
    rand::prelude::SliceRandom,
};

const W: u32 = 950;
const H: u32 = 950;

macro_rules! from255 {
    ($a:expr, $b:expr, $c:expr) => {
        rgb($a as f32 / 255.0, $b as f32 / 255.0, $c as f32 / 255.0)
    };
}

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
    //let img_path = app.assets_path().unwrap().join("image-00.png");

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

fn brightness(r: f32, g: f32, b: f32) -> f32 {
    vec3(r, g, b).length()
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let palette = vec![
        from255!(46, 41, 78),
        from255!(84, 19, 136),
        from255!(241, 233, 218),
        from255!(255, 212, 0),
        from255!(217, 3, 104),
    ];

    let target = from255!(255, 220, 0);

    let dist = palette
        .iter()
        .map(|c| {
            let c: Hsl<Srgb> = c.into_hsl();
            let v0 = vec3(c.hue.to_degrees(), c.lightness, c.saturation);
            let t: Hsl<Srgb> = target.into_hsl();
            let v1 = vec3(t.hue.to_degrees(), t.lightness, t.saturation);

            1.0 / v0.distance(v1)
        })
        .collect::<Vec<_>>();

    let mut rng = nannou::rand::thread_rng();

    //draw.background().color(*palette.choose(&mut rng).unwrap());
    //draw.background().color(palette[0]);
    draw.background().color(BLACK);

    let b = W as f32 * 0.45;

    for _ in 0..2_000 {
        let _r = random_range(2.0f32, 8.0);

        let (x, y) = (random_range(-b, b), random_range(-b, b));

        let mut tar = target;
        if x > 0.0 && y > 0.0 {
            tar = from255!(0, 0, 255);
        } else if x > 0.0 && y < 0.0 {
            tar = from255!(255, 0, 0);
        } else if x < 0.0 && y < 0.0 {
            tar = from255!(0, 255, 0);
        }

        let col_dist = |c: &Rgb<f32>| {
            let c: Hsl<Srgb> = c.into_hsl();
            let v0 = vec3(c.hue.to_degrees(), c.lightness, c.saturation);
            let t: Hsl<Srgb> = tar.into_hsl();
            let v1 = vec3(t.hue.to_degrees(), t.lightness, t.saturation);

            1.0 / v0.distance(v1)
        };

        let col = palette.choose_weighted(&mut rng, col_dist).unwrap();

        draw.ellipse()
            .w_h(1.8, 1.8)
            .x_y(x, y)
            //.color(*palette.choose(&mut rng).unwrap());
            .color(*col);
    }

    draw.to_frame(app, &frame).unwrap();
}
