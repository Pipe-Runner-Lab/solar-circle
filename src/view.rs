use crate::prelude::*;

pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let c_draw = draw
        .x_y(-1.0 * (WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32)
        .scale_y(-1.0);

    c_draw.ellipse().color(STEELBLUE).radius(50.0).x_y((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32);

    c_draw.to_frame(app, &frame).unwrap();
}
