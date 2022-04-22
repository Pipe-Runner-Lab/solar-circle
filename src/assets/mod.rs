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
    pub polar_radius_1: f32,
    pub planet_1_radius: f32,
    pub polar_radius_2: f32,
    pub planet_2_radius: f32,
    pub polar_radius_3: f32,
    pub planet_3_radius: f32,
    pub polar_radius_4: f32,
    pub planet_4_radius: f32,
}
