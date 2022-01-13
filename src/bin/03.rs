use std::time::SystemTime;

use genuary_2022::*;

use nannou::{
    image::buffer::PixelsMut,
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

const W: u32 = 800;
const H: u32 = 800;

const B_W: f32 = (W / 2 - 50) as f32;
const B_H: f32 = (H / 2 - 50) as f32;

const NOISE_SCALE: f64 = 25.3;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    noise: Perlin,
    zoom: f32,
    particles: Vec<Dust>,
}

struct Dust {
    xy: Vec2,
    vel: Vec2,
}

fn dust(xy: Vec2) -> Dust {
    Dust {
        xy,
        vel: vec2(0.0, 0.0),
    }
}

fn event_update(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseWheel(delta, _) => match delta {
            MouseScrollDelta::LineDelta(_, v) => {
                if v > 0.0 {
                    model.zoom *= 2.0
                } else {
                    model.zoom /= 2.0
                }
            }
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

    app.main_window().set_inner_size_pixels(W, H);

    let noise = Perlin::new();
    let noise = noise.set_seed(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_millis(),
    );

    let mut particles = Vec::new();

    for _ in 0..10_000 {
        particles.push(dust(vec2(
            random_range(-40.0, 40.0),
            random_range(-40.0, 40.0),
        )))
    }

    Model {
        _window,
        noise,
        zoom: 25.0,
        particles,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for Dust { xy, vel } in model.particles.iter_mut() {
        *xy += *vel;
        *vel *= 0.685;

        if !app.main_window().rect().contains((*xy) * model.zoom) {
            *xy = vec2(random_range(-40.0, 40.0), random_range(-40.0, 40.0));
        }

        let noise_val = model
            .noise
            .get([(xy.x as f64) / NOISE_SCALE, (xy.y as f64) / NOISE_SCALE]);

        //let c = (noise_val + 1.0) / 2.0;
        let angle = noise_val as f32 * 2.0 * PI;

        let k = 0.01;
        vel.x += angle.cos() * k;
        vel.y += angle.sin() * k;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    //draw.background().color(BLACK);
    //draw.background().color(rgba(0.1, 0.1, 0.1, 0.001));

    let scale = model.zoom;

    //if app.elapsed_frames() < 2 {
    //    for i in (-10)..(10) {
    //        for j in (-10)..(10) {
    //            let noise_val = model
    //                .noise
    //                .get([(i as f64) / NOISE_SCALE, (j as f64) / NOISE_SCALE]);

    //            let c = (noise_val + 1.0) / 2.0;
    //            let angle = noise_val as f32 * 2.0 * PI;
    //            draw.rect()
    //                .wh(vec2(0.9 * scale, 0.9 * scale))
    //                .xy(vec2(i as f32 * scale, j as f32 * scale))
    //                .color(rgb(c / 1.5, 0.1, 0.1));

    //            draw.line()
    //                .start(vec2(
    //                    i as f32 * scale + (angle.cos() * scale) / 2.3,
    //                    j as f32 * scale + (angle.sin() * scale) / 2.3,
    //                ))
    //                .end(vec2(i as f32 * scale, j as f32 * scale))
    //                .color(GREY);

    //            draw.ellipse()
    //                .wh(vec2(scale / 10.0, scale / 10.0))
    //                .xy(vec2(
    //                    i as f32 * scale + (angle.cos() * scale) / 2.3,
    //                    j as f32 * scale + (angle.sin() * scale) / 2.3,
    //                ))
    //                .color(GREY);
    //        }
    //    }
    //}

    for Dust { xy, .. } in &model.particles {
        draw.ellipse()
            .wh(vec2(scale / 15.0, scale / 15.0))
            .xy((*xy) * scale)
            .color(rgba(0.2, 0.7, 0.9, 0.005));
        //.color(WHITE);
    }

    //model.noise.get(point)

    draw.to_frame(app, &frame).unwrap();
    //app.save_animation();
}
