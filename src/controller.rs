use crate::prelude::*;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update control panel data
    update_control_panel(model);

    // Update art data
    for asset in &mut model.assets {
        match asset {
            ASSETS::SUN(sun) => {
                sun.radius = model.asset_metrics.sun_radius;
            },
            ASSETS::ORBIT(orbit) => {
                // do something
            }
            ASSETS::PLANET(planet) => {
                // do something
            }
        }
    }
}

fn update_control_panel(model: &mut Model) {
    // TODO: control update
    // todo!()

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
