mod controller;
mod model;
mod view;
mod physics_engine;
mod assets;

mod prelude {
    pub use nannou::prelude::*;

    pub use crate::controller::*;
    pub use crate::model::*;
    pub use crate::view::*;
    pub use crate::physics_engine::*;
    pub use crate::assets::*;

    pub const WINDOW_WIDTH: u32 = 1080;
    pub const WINDOW_HEIGHT: u32 = 1080;
}

use prelude::*;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
