use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    bg_color: Srgb<u8>,
    fg_color: Srgb<u8>,
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        bg_color: WHITE,
        fg_color: STEELBLUE,
        x: 0.0,
        y: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let sin = app.time.sin();
    let sin2 = (app.time / 2.0).sin();
    let window = app.window_rect();
    model.x = map_range(sin, -1.0, 1.0, window.left(), window.right());
    model.y = map_range(sin2, -1.0, 1.0, window.bottom(), window.top());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.bg_color);
    draw.ellipse().color(model.fg_color).x_y(model.x, model.y);

    draw.to_frame(app, &frame).unwrap();
    capture_frame(app, frame);
}

fn capture_frame(app: &App, frame: Frame) {
    let file_path = app
        .project_path()
        .unwrap()
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png");
    app.main_window().capture_frame(file_path);
}
