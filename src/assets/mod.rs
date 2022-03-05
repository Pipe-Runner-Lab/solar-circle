mod sun;
mod orbit;
mod planet;

use crate::prelude::*;
pub use self::sun::*;
pub use self::orbit::*;
pub use self::planet::*;

// INFO: Not making something pub means it can be 'imported' (but import statement will still be needed)
// to work across modules
pub trait Asset2D {
    fn tick(&self, ctx: &Draw);
}

pub enum ASSETS {
    SUN(Sun),
    ORBIT(Orbit),
    PLANET(Planet)
}