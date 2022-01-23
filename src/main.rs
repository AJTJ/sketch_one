use nannou::prelude::*;
use rand::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        // this makes it tiny, for when it first appears on-screen so as not to distract
        .size(1, 1)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    // list available monitors
    // let r = app.available_monitors();
    // println!("{:?}", r);

    // move it to the corner of the screen
    app.main_window().set_outer_position_pixels(0, 0);
    // resize it so that it is now visible
    app.main_window().set_inner_size_pixels(1000, 1000);

    // set it fullscreen
    // app.main_window().set_fullscreen(true);
}

fn get_color(time: f32) -> Rgba {
    let mut rng = rand::thread_rng();
    // let ran: f32 = rng.gen_range(0..100) as f32;
    // println!("{}", ran);
    rgba(
        // (rng.gen_range(0..1000) as f32) / 1000.0,
        // (rng.gen_range(0..1000) as f32) / 1000.0,
        // (rng.gen_range(0..1000) as f32) / 1000.0,
        (time % 1000.0) / 1000.0,
        (time % 500.0) / 1000.0,
        (time % 800.0) / 1000.0,
        1.0,
    )
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // GENERAL
    let draw = app.draw();

    // CIRCLE ANIMATION
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();

    let sine_x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let slow_sine_y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // let mut color = rgba(0.0, 0.5, 0.63, 1.0);
    // let color = srgb(0.10, 0.10, 0.10);
    // let color = hsl(0.5, 1.0, 0.5);
    // this also consistently paints the background
    // draw.background().color(PLUM);
    draw.ellipse()
        .color(get_color(app.duration.since_start.as_millis() as f32))
        .x_y(sine_x, slow_sine_y);

    draw.to_frame(app, &frame).unwrap();
}
// dank memes
