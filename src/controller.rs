use crate::prelude::*;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    // Update control panel data
    update_control_panel(model);

    // Update art data with control panel settings
    for asset in &mut model.assets {
        match asset {
            ASSETS::SUN(sun) => {
                sun.radius = model.asset_metrics.sun_radius;
            },
            ASSETS::PLANET(planet) => {
                
            },
            _ => {
                // Do nothing
            }
        }
    }
}
