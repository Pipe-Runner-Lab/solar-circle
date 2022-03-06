use crate::prelude::*;

pub fn view_art(app: &App, model: &Model, frame: Frame) {
    // TODO: Is recreating context a good idea?
    let draw = app.draw();

    // INFO: Creating new context to match HTML Canvas co-ordinate system
    let ctx = draw
        .x_y(-1.0 * (WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32)
        .scale_y(-1.0);

    // Setting background color
    ctx.background().color(hsl(
        BACKGROUND_COLOR.0,
        BACKGROUND_COLOR.1,
        BACKGROUND_COLOR.2,
    ));

    // TODO: Confusing reference here
    for asset in &model.assets {
        match &asset {
            ASSETS::SUN(sun) => {
                sun.tick(&ctx);
            },
            ASSETS::ORBIT(orbit) => {
                orbit.tick(&ctx);
            }
            ASSETS::PLANET(planet) => {
                planet.tick(&ctx);
            }
        }
    }

    ctx.to_frame(app, &frame).unwrap();
}

pub fn view_control_panel(app: &App, model: &Model, frame: Frame) {
    model.control_panel.draw_to_frame_if_changed(app, &frame).unwrap();
}
