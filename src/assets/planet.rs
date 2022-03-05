use crate::prelude::*;

pub struct Planet {
    pub color: (f32, f32, f32),
    pub radius: f32,
    /// Angle has to be in degrees from 0.0 to 360.0
    pub polar_angle: f32,
    pub polar_radius: f32,
}

impl Asset2D for Planet {
    fn tick(&self, ctx: &Draw) {
        ctx.ellipse()
            .radius(self.radius)
            .color(hsl(self.color.0, self.color.1, self.color.2))
            .x_y(
                (WINDOW_WIDTH / 2) as f32 + self.polar_radius * self.polar_angle.cos(),
                (WINDOW_HEIGHT / 2) as f32 + self.polar_radius * self.polar_angle.sin(),
            );
    }
}
