mod assets;
mod controller;
mod model;
mod physics_engine;
mod view;
mod events;

mod prelude {
    pub use nannou::prelude::*;
    pub use nannou_conrod::prelude::*;

    pub use crate::assets::*;
    pub use crate::controller::*;
    pub use crate::model::*;
    pub use crate::physics_engine::*;
    pub use crate::view::*;
    pub use crate::events::*;

    pub const WINDOW_WIDTH: u32 = 1080;
    pub const WINDOW_HEIGHT: u32 = 1080;
    // https://docs.rs/nannou/latest/nannou/color/struct.Hsl.html
    pub const BACKGROUND_COLOR: (f32, f32, f32) = (206.0 / 360.0, 50.0 / 100.0, 1.0 / 100.0);
    
    pub const SUN_COLOR: (f32, f32, f32) = (43.0 / 360.0, 100.0 / 100.0, 57.0 / 100.0);
    
    pub const ORBIT_1_COLOR: (f32, f32, f32) = (56.0 / 360.0, 85.0 / 100.0, 89.0 / 100.0);
    pub const ORBIT_2_COLOR: (f32, f32, f32) = (35.0 / 360.0, 100.0 / 100.0, 91.0 / 100.0);
    pub const ORBIT_3_COLOR: (f32, f32, f32) = (185.0 / 360.0, 84.0 / 100.0, 76.0 / 100.0);
    pub const ORBIT_4_COLOR: (f32, f32, f32) = (126.0 / 360.0, 89.0 / 100.0, 85.0 / 100.0);
}

use prelude::*;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
