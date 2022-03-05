use crate::prelude::*;

use super::Asset2D;

pub struct Orbit {
    pub radius: f32,
    pub color: (f32, f32, f32),
}

impl Asset2D for Orbit {
    fn tick(&self, ctx: &nannou::Draw) {
        ctx.ellipse()
            .radius(self.radius)
            .x_y((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32)
            .stroke(hsla(self.color.0, self.color.1, self.color.2, 0.3))
            .color(hsla(0.0, 0.0, 0.0, 0.0))
            .stroke_weight(1.0);
    }
}
