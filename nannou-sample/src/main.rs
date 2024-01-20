use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    bg_color: Srgb<u8>,
    fg_color: Srgb<u8>,
    step_length: f32,
    start: Point2,
    end: Point2,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        bg_color: WHITE,
        fg_color: STEELBLUE,
        step_length: 10.0,
        start: pt2(0.0, 0.0),
        end: pt2(0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // 前回の終点を始点にする
    model.start = model.end;

    // 一歩の移動増分を求める
    let angle = random_range(0.0, 2.0 * PI);
    let vec = vec2(angle.cos(), angle.sin()) * model.step_length;

    // 一歩進んだ先の点を求める
    model.end = model.start + vec;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // 最初だけ背景を描画
    if app.elapsed_frames() == 0 {
        draw.background().color(model.bg_color);
    }

    // ランダムウォークの一歩を描画
    draw.line()
        .color(model.fg_color)
        .start(model.start)
        .end(model.end);

    draw.to_frame(app, &frame).unwrap();
}
