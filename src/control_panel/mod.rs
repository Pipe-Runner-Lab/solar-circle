use crate::prelude::*;

// This defines the type of widgets the panel
// should support and the order in which they should
// appear
widget_ids! {
  pub struct Ids {
      title,
      sun_radius_label,
      sun_radius_slider,
      randomize,
      seed_label,
      seed_text,
  }
}

/// Sets up control panel and returns ids and the control panel object
pub fn control_panel_setup(app: &App) -> (Ids, Ui ) {
    // INFO: Setup control panel
    // https://github.com/sidwellr/schotter/blob/main/schotter3/src/main.rs
    let control_panel_window = app
        .new_window()
        .title(app.exe_name().unwrap() + "controls")
        .size(300, 200)
        .view(view_control_panel)
        .raw_event(raw_ui_event)
        .build()
        .unwrap();

    let mut control_panel = nannou_conrod::builder(app)
        .window(control_panel_window)
        .build()
        .unwrap();
    let control_panel_widget_ids = Ids::new(control_panel.widget_id_generator());

    // Set control panel theme
    control_panel.clear_with(color::DARK_CHARCOAL);
    let mut theme = control_panel.theme_mut();
    theme.label_color = color::WHITE;
    theme.shape_color = color::CHARCOAL;

    (control_panel_widget_ids, control_panel)
}

/// view function for control panel window
pub fn view_control_panel(app: &App, model: &Model, frame: Frame) {
    model
        .control_panel
        .draw_to_frame_if_changed(app, &frame)
        .unwrap();
}

/// Update / controller function for control panel
pub fn update_control_panel(model: &mut Model) {
    let ui = &mut model.control_panel.set_widgets();

    // Control panel title
    widget::Text::new("Solar Circle / Control Panel")
        .top_left_with_margin(10.0)
        .w_h(300.0, 40.0)
        .font_size(20)
        .set(model.control_panel_widget_ids.title, ui);

    // Sun Radius label
    widget::Text::new("Sun Radius")
        .down_from(model.control_panel_widget_ids.title, 10.00)
        .w_h(125.0, 30.0)
        .set(model.control_panel_widget_ids.sun_radius_label, ui);

    // Sun Radius slider
    for value in widget::Slider::new(model.asset_metrics.sun_radius, 10.0, 100.0)
        .right_from(model.control_panel_widget_ids.sun_radius_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.asset_metrics.sun_radius.to_string())
        .set(model.control_panel_widget_ids.sun_radius_slider, ui)
    {
        model.asset_metrics.sun_radius = value;
    }

    // // Randomize button
    // for _click in widget::Button::new()
    //     .down_from(model.control_panel_widget_ids.rot_label, 15.0)
    //     .w_h(125.0, 40.0)
    //     .label("Randomize")
    //     .set(model.control_panel_widget_ids.randomize, ui)
    // {
    //     model.random_seed = random_range(0, 1000000);
    // }

    // // Seed label
    // widget::Text::new("Seed")
    //     .right_from(model.control_panel_widget_ids.randomize, 10.0)
    //     .w_h(40.0, 30.0)
    //     .set(model.control_panel_widget_ids.seed_label, ui);

    // // Seed text
    // for event in widget::TextBox::new(&model.random_seed.to_string())
    //     .right_from(model.control_panel_widget_ids.seed_label, 10.0)
    //     .w_h(100.0, 30.0)
    //     .set(model.control_panel_widget_ids.seed_text, ui)
    // {
    //     use nannou_conrod::widget::text_box::Event;
    //     match event {
    //         Event::Update(seed) => {
    //             model.random_seed = seed.parse().unwrap_or(0);
    //         }
    //         Event::Enter => {}
    //     }
    // }
}
