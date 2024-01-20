use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    draw.ellipse()
        .color(STEELBLUE)
        .w_h(300.0, 200.0)
        .x_y(200.0, 150.0);

    draw.to_frame(app, &frame).unwrap();
}
