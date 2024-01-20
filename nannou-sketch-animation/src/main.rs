use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let sin = app.time.sin();
    let sin2 = (app.time / 2.0).sin();
    let window = app.window_rect();
    let x = map_range(sin, -1.0, 1.0, window.left(), window.right());
    let y = map_range(sin2, -1.0, 1.0, window.bottom(), window.top());

    draw.background().color(WHITE);
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
