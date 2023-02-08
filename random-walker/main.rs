use nannou::prelude::*;

const BG_COLOR: (u8, u8, u8, u8) = (0x28, 0x28, 0x28, 1);
const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 400;
const WALKER_SATURATION: f32 = 50.0;
const WALKER_BRIGHTNESS: f32 = 80.0;
const WALKER_ALPHA: f32 = 1.0;
const STROKE_WEIGHT: f32 = 2.0;
const STEP_MIN: f32 = -5.0;
const STEP_MAX: f32 = 5.0;

struct Walker {
    position: Point2,
    prev_position: Point2,
    color: Hsva,
}

impl Walker {
    fn new() -> Self {
        let initial_hue = map_range(
            0.0f32,
            -(WINDOW_HEIGHT as f32) / 2.0,
            (WINDOW_HEIGHT as f32) / 2.0,
            0.0,
            360.0,
        );

        Walker {
            position: Point2::ZERO,
            prev_position: Point2::ZERO,
            color: Hsva::new(
                initial_hue,
                WALKER_SATURATION,
                WALKER_BRIGHTNESS,
                WALKER_ALPHA,
            ),
        }
    }

    fn walk(&mut self) {
        let new_pos = Point2::new(
            random_range(STEP_MIN, STEP_MAX),
            random_range(STEP_MIN, STEP_MAX),
        );

        self.prev_position = self.position;
        self.position += new_pos;

        let new_hue = map_range(
            self.position.y,
            -(WINDOW_HEIGHT as f32) / 2.0,
            (WINDOW_HEIGHT as f32) / 2.0,
            0.0,
            360.0,
        );

        self.color.hue = new_hue.into();
    }
}

fn model(_app: &App) -> Walker {
    Walker::new()
}

fn setup(draw: &Draw, frame: &Frame) {
    if frame.nth() == 0 {
        draw.background().color(Rgba8::from_components(BG_COLOR));
    }
}

fn update(_app: &App, model: &mut Walker, _update: Update) {
    model.walk()
}

fn view(app: &App, model: &Walker, frame: Frame) {
    let draw = app.draw();

    setup(&draw, &frame);

    draw.line()
        .points(model.prev_position, model.position)
        .weight(STROKE_WEIGHT)
        .color(model.color);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .simple_window(view)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .update(update)
        .run();
}
