mod orbit;
mod planet;
mod sun;

pub use self::orbit::*;
pub use self::planet::*;
pub use self::sun::*;
use crate::prelude::*;

// INFO: Not making something pub means it can be 'imported' (but import statement will still be needed)
// to work across modules
pub trait Asset2D {
    fn tick(&self, ctx: &Draw);
}

pub enum ASSETS {
    SUN(Sun),
    ORBIT(Orbit),
    PLANET(Planet),
}

pub struct AssetMetrics {
    pub sun_radius: f32,
}

impl AssetMetrics {

    // TODO: This is redundant if sun_radius is public
    pub fn set_sun_radius(&mut self, radius: f32) {
        self.sun_radius = radius;
    }
}
