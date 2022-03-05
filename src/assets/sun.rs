use crate::prelude::*;

use super::Asset2D;

pub struct Sun {}

impl Asset2D for Sun {
    fn tick(&self, ctx: &Draw) {
        ctx.ellipse()
            .color(hsl(SUN_COLOR.0, SUN_COLOR.1, SUN_COLOR.2))
            .radius(50.0)
            .x_y((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32);
    }
}
