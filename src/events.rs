use crate::prelude::*;

pub fn raw_ui_event(app: &App, model: &mut Model, event: &nannou_conrod::RawWindowEvent) {
  model.control_panel.handle_raw_event(app, event);
}

// TODO: Key binds handlers