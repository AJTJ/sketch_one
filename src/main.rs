use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    app.new_window();

    let r = app.available_monitors();
    println!("{:?}", r);

    // setting fullscreen, but stays on same monitor
    // app.main_window()
    //     .set_fullscreen_with(Some(Fullscreen::Borderless(None)));

    // to corner of screen, but still steals focus
    // app.main_window().set_outer_position_pixels(0, 0);
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
// dank memes
