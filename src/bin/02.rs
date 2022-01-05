use genuary_2022::SaveAnimation;
// Dithering
use nannou::prelude::*;

const W: u32 = 800;
const H: u32 = 800;
const R: f32 = 15.0;
const R_inner: f32 = 1.0;

const B_W: f32 = (W / 2 - 50) as f32;
const B_H: f32 = (H / 2 - 50) as f32;

struct Circle {
    xy: nannou::geom::Vec2,
    r: f32,
}

fn circ(xy: Vec2, r: f32) -> Circle {
    Circle { xy, r }
}

fn valid_point(p: &Circle, pts: &[Circle]) -> bool {
    pts.iter()
        .all(|Circle { xy, r }| p.xy.distance(*xy) > *r + p.r)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Circle>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    app.main_window().set_inner_size_pixels(W, H);
    //app.set_loop_mode(LoopMode::loop_once());

    //let points = vec![circ(vec2(10.0, 10.0), R)];

    Model {
        _window,
        points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _ in 0..100_000 {
        let mut point = circ(vec2(random_range(-B_W, B_W), random_range(-B_H, B_H)), R);

        //point.r = (point.xy.x.abs().log2() / 1.0)
        //    .min(point.xy.y.abs().log2() / 1.0)
        //    .min((B_W - point.xy.x.abs()).log2() / 1.5)
        //    .min((B_H - point.xy.y.abs()).log2() / 1.5)
        //    .max(R_inner / 2.2);

        point.r = ((B_W - point.xy.x.abs()).log2() / 2.5)
            .min((B_H - point.xy.y.abs()).log2() / 2.5)
            .max(R_inner / 2.2);
        
        let ball_r = 250.0;
        let d = point.xy.length();

        //point.r = if d > (ball_r+10.0) {
        //    10.0
        //} else {
        //    (ball_r - d).log2().max(R_inner / 2.2)
        //};

        if valid_point(&point, &model.points) {
            model.points.push(point);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DARKORANGE);

    for Circle { xy, .. } in &model.points {
        draw.ellipse().xy(*xy).radius(R_inner).color(BLACK);

        //draw.ellipse()
        //    .xy(*xy)
        //    .no_fill()
        //    .stroke_weight(1.2)
        //    .stroke_color(GREY)
        //    .radius(*r);
    }

    draw.to_frame(app, &frame).unwrap();
    app.save_animation();
}
