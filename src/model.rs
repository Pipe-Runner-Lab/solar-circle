use crate::prelude::*;

pub struct Model {
    pub window: window::Id,
    pub sun: Sun
}

impl Model {
    /// This function creates a new model instance
    pub fn new(app: &App) -> Self {
        let sun = Sun {};

        let window = app
            .new_window()
            .view(view_art)
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()
            .unwrap();
        Model { window, sun }
    }
}
