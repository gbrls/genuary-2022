// Isometric perspective.

use genuary_2022::*;
use nannou::{
    color::{encoding::Srgb, IntoColor},
    image::{DynamicImage, GenericImageView},
    prelude::*,
    rand::prelude::SliceRandom,
};
use std::cmp::Ordering;

const W: u32 = 800;
const H: u32 = 800;

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
    //draw.background().color(hsl(0.0, 0.0, 0.4));
    draw.background().color(BLACK);

    let palette = vec![hsl(0.4, 1.0, 1.0), hsl(0.9, 1.0, 0.5), hsl(0.0, 1.0, 0.5)];

    let palette = (0..16)
        .map(|_| {
            hsl(
                random_range(0.0, 1.0),
                random_range(0.1, 0.9),
                random_range(0.0, 1.0),
            )
        })
        .collect::<Vec<_>>();

    let mut rng = nannou::rand::thread_rng();

    let (n, m) = (22, 22);

    let sclx = (W as f32 / n as f32);
    let scly = (H as f32 / m as f32);

    let mut center_pts = Vec::new();

    for i in 0..=n {
        for j in 0..=m {
            let i = if j % 2 == 0 { i as f32 } else { i as f32 + 0.5 };

            let pt = vec2(
                i as f32 * (sclx * 1.0) - (W as f32 * 0.5),
                j as f32 * (scly * 1.0) - (H as f32 * 0.5),
            );

            center_pts.push(pt);
        }
    }

    center_pts.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap_or(Ordering::Equal));

    for pt in center_pts {
        let dirs = vec![
            vec2(0.0, 0.0),
            vec2(0.0, -1.0),
            vec2(1.0, 1.0).normalize(),
            vec2(-1.0, 1.0).normalize(),
        ];

        let pts = dirs
            .into_iter()
            .map(|dir| {
                pt + vec2(0.0, random_range(0.0, 10.0)) + dir * vec2(sclx * 0.75, scly * 0.75)
            })
            .collect::<Vec<_>>();

        let p0 = pts[0];
        for p1 in pts.iter().skip(1) {
            for p2 in pts.iter().skip(2) {
                let p1 = *p1;
                let p2 = *p2;

                let p3 = (p1 - p0) + (p2 - p0) + p0;
                let mut ps = vec![p0, p1, p2, p3];

                let pm = (p0 + p1 + p2 + p3) / 4.0;

                let ang = |p: &Vec2| (*p - pm).angle();

                ps.sort_by(|a, b| ang(a).partial_cmp(&ang(b)).unwrap_or(Ordering::Equal));

                draw.polygon()
                    .points(ps.into_iter())
                    .color(*palette.choose(&mut rng).unwrap());

                //for p in &ps {
                //    draw.ellipse().w_h(5.0, 5.0).xy(*p).color(RED);
                //}
            }

            draw.line().start(p0).end(*p1).weight(2.0).color(hsla(
                0.0,
                0.0,
                random_range(0.25, 0.35),
                random_range(0.3, 0.85),
            ));
        }

        // draw.ellipse()
        //     .w_h(5.0, 5.0)
        //     .x_y((i as f32 + off) * scl, j as f32 * scl)
        //     .color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
