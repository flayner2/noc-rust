use nannou::prelude::*;

struct Walker {
    x: f32,
    y: f32,
    color: Hsva,
}

fn hsva_lerp(a: &Hsva, b: &Hsva, t: f32) -> Hsva {
    let mut t = t;
    let mut hue;
    let mut a_hue = a.hue.to_positive_degrees();
    a_hue = map_range(a_hue, 0.0, 360.0, 0.0, 1.0);
    let mut b_hue = b.hue.to_positive_degrees();
    b_hue = map_range(b_hue, 0.0, 360.0, 0.0, 1.0);

    let mut d = b_hue - a_hue;

    if a_hue > b_hue {
        let hue_3 = b_hue;
        b_hue = a_hue;
        a_hue = hue_3;

        d = -d;
        t = 1.0 - t;
    }

    if d > 0.5 {
        a_hue += 1.0;
        hue = (a_hue + t * (b_hue - a_hue)) % 1.0;
    } else {
        hue = a_hue + t * d;
    }

    let saturation = a.saturation + t * (b.saturation - a.saturation);
    let value = a.value + t * (b.value - a.value);
    let alpha = a.alpha + t * (b.alpha - a.alpha);

    hue = map_range(hue, 0.0, 1.0, 0.0, 360.0);

    Hsva::new(hue, saturation, value, alpha)
}

fn model(_app: &App) -> Walker {
    Walker {
        x: 0.0,
        y: 0.0,
        color: Hsva::new(0.0, 1.0, 1.0, 1.0),
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
    let blue = Hsva::new(240.0, 1.0, 1.0, 1.0);
    let new_color = hsva_lerp(&model.color, &blue, 0.01);

    //model.x += new_x;
    //model.y += new_y;
    //model.color = new_color;
}

fn view(app: &App, model: &Walker, frame: Frame) {
    let draw = app.draw();

    // Only runs once (on the first frame)
    //setup(&draw, &frame);

    draw.background().color(WHITE);

    let hue = map_range(model.color.hue.to_positive_degrees(), 0.0, 360.0, 0.0, 1.0);

    draw.rect()
        .xy(pt2(model.x, model.y))
        .wh(vec2(5.0, 5.0))
        .hsva(
            hue,
            model.color.saturation,
            model.color.value,
            model.color.alpha,
        );

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).simple_window(view).update(update).run();
}
