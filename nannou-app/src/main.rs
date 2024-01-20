use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
    bg_color: Srgb<u8>,
    fg_color: Srgb<u8>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        bg_color: WHITE,
        fg_color: STEELBLUE,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.bg_color);
    draw.ellipse().color(model.fg_color);

    draw.to_frame(app, &frame).unwrap();
}
