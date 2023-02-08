use nannou::prelude::*;

struct Walker {
    x: f32,
    y: f32,
    color: Hsva,
}

fn model(_app: &App) -> Walker {
    Walker {
        x: 0.0,
        y: 0.0,
        color: Hsva::new(0.0, 0.0, 0.0, 1.0),
    }
}

fn setup(draw: &Draw, frame: &Frame) {
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
}

fn update(_app: &App, model: &mut Walker, _update: Update) {
    let new_x = random_range::<f32>(-1.0, 1.0);
    let new_y = random_range::<f32>(-1.0, 1.0);

    model.x += new_x;
    model.y += new_y;
}

fn view(app: &App, model: &Walker, frame: Frame) {
    let draw = app.draw();

    setup(&draw, &frame);

    draw.rect()
        .xy(pt2(model.x, model.y))
        .wh(vec2(2.0, 2.0))
        .color(model.color);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}
