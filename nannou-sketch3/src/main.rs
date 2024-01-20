use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    draw.tri().color(STEELBLUE).points(
        pt2(150.0, 200.0),
        pt2(-100.0, 300.0),
        pt2(-50.0, -150.0),
    );

    draw.to_frame(app, &frame).unwrap();
}
