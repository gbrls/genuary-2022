use std::{iter, time::SystemTime};

use genuary_2022::*;

use nannou::{
    image::buffer::PixelsMut,
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
    rand::{prelude::SliceRandom, thread_rng},
    text::lines,
};

const W: u32 = 800;
const H: u32 = 800;

const B_W: f32 = (W / 2 - 50) as f32;
const B_H: f32 = (H / 2 - 50) as f32;

const NOISE_SCALE: f64 = 150.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    noise: Perlin,
    zoom: f32,

    particles: Vec<Dust>,
    lines: Vec<Vec<Vec2>>,
}

struct Dust {
    xy: Vec2,
    vel: Vec2,
    lifetime: u32,
}

fn dust(xy: Vec2) -> Dust {
    Dust {
        xy,
        vel: vec2(0.0, 0.0),
        lifetime: random_range(20, 80),
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

    for _ in 0..1 {
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
        lines: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for Dust { xy, vel, lifetime } in model.particles.iter_mut() {
        match model.lines.last_mut() {
            Some(last) => last.push((*xy) * model.zoom),
            None => model.lines.push(vec![*xy * model.zoom]),
        }

        *xy += (*vel) * 0.35;
        if !vel.x.is_zero() && !vel.y.is_zero() {
            *vel = vel.normalize();
        }

        if *lifetime == 0 || !app.main_window().rect().contains((*xy) * model.zoom) {
            *xy = vec2(random_range(-40.0, 40.0), random_range(-40.0, 40.0));
            *lifetime = random_range(10, 150);

            let valid = if model.lines.len() == 1 {
                true
            } else {
                valid_line(
                    model.lines.last().unwrap(),
                    &model.lines[0..(model.lines.len() - 1).max(0)],
                )
            };

            match valid {
                true => model.lines.push(Vec::new()),
                false => model.lines.last_mut().unwrap().clear(),
            }
        }

        let noise_val = model
            .noise
            .get([(xy.x as f64) / NOISE_SCALE, (xy.y as f64) / NOISE_SCALE]);

        //let c = (noise_val + 1.0) / 2.0;
        let angle = noise_val as f32 * 2.0 * PI;

        let k = 0.95;
        //vel.x += angle.cos() * k;
        //vel.y += angle.sin() * k;
        //
        vel.x = angle.cos() * k;
        vel.y = angle.sin() * k;

        *lifetime -= 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    //draw.background().color(rgba(0.1, 0.1, 0.1, 1.0));
    //

    let scale = model.zoom;

    //if app.elapsed_frames() & (1 << 7) != 0 {
    if false {
        for i in (-10)..(10) {
            for j in (-10)..(10) {
                let noise_val = model
                    .noise
                    .get([(i as f64) / NOISE_SCALE, (j as f64) / NOISE_SCALE]);

                let c = (noise_val + 1.0) / 2.0;
                let angle = noise_val as f32 * 2.0 * PI;
                draw.rect()
                    .wh(vec2(0.9 * scale, 0.9 * scale))
                    .xy(vec2(i as f32 * scale, j as f32 * scale))
                    .color(rgb(c / 1.5, 0.1, 0.1));

                draw.line()
                    .start(vec2(
                        i as f32 * scale + (angle.cos() * scale) / 2.3,
                        j as f32 * scale + (angle.sin() * scale) / 2.3,
                    ))
                    .end(vec2(i as f32 * scale, j as f32 * scale))
                    .color(GREY);

                draw.ellipse()
                    .wh(vec2(scale / 10.0, scale / 10.0))
                    .xy(vec2(
                        i as f32 * scale + (angle.cos() * scale) / 2.3,
                        j as f32 * scale + (angle.sin() * scale) / 2.3,
                    ))
                    .color(GREY);
            }
        }
    }

    let scale = model.zoom;

    let pallete = [(3, 36, 128), (64, 86, 148), (91, 76, 117)]
        .map(|(r, g, b)| rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0));

    let mut rng = thread_rng();
    for line in &model.lines {
        if !line.is_empty() {
            let line_c: Vec<_> = line
                .iter()
                .map(|x| (*x, *pallete.choose(&mut rng).unwrap()))
                .collect();
            draw.polyline()
                .stroke_weight(8.0)
                //.points_colored((line_c, colours))
                .points_colored(line_c)
                //.color(rgba(1.0, 1.0, 1.0, 0.1));
                .color(WHITESMOKE);
        }
    }

    for Dust { xy, .. } in &model.particles {
        draw.ellipse()
            .wh(vec2(scale / 5.0, scale / 5.0))
            .xy((*xy) * scale)
            .color(rgba(0.2f32, 0.7, 0.9, 0.8));
    }

    //model.noise.get(point)

    draw.to_frame(app, &frame).unwrap();
    //app.save_animation();
}

fn valid_line(line: &[Vec2], lines: &[Vec<Vec2>]) -> bool {
    line.iter().all(|u| {
        lines
            .iter()
            .all(|old_line| old_line.iter().all(|v| u.distance(*v) > 1.0))
    })
}
