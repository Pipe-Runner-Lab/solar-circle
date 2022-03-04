mod model;
mod view;
mod controller;

mod prelude {
    pub use nannou::prelude::*;
    pub use crate::model::*;
    pub use crate::view::*;
    pub use crate::controller::*;

    pub const WIDTH: u32 = 1080;
    pub const HEIGHT: u32 = 1080;
}

use crate::prelude::*;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}
