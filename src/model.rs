use crate::prelude::*;

pub struct Model {
    pub _window: window::Id,
}

impl Model {
    /// This function creates a new model instance
    pub fn new(app: &App) -> Model {
        let _window = app
            .new_window()
            .view(view)
            .size(WIDTH, HEIGHT)
            .build()
            .unwrap();
        Model { _window }
    }
}
