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

pub struct Model {
    pub main_window: window::Id,
    pub assets: Vec<ASSETS>,
    pub control_panel: Ui,
    pub control_panel_widget_ids: Ids,
    pub asset_metrics: AssetMetrics,
}

impl Model {
    /// This function creates a new model instance
    pub fn new(app: &App) -> Self {
        let asset_metrics = AssetMetrics { sun_radius: 50.0 };

        let mut assets = Vec::new();

        assets.push(ASSETS::SUN(Sun {
            radius: asset_metrics.sun_radius,
        }));
        assets.push(ASSETS::ORBIT(Orbit {
            radius: 130.0,
            color: ORBIT_1_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_1_COLOR,
            radius: 20.0,
            polar_radius: 130.0,
            polar_angle: 45.0,
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 200.0,
            color: ORBIT_2_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_2_COLOR,
            radius: 10.0,
            polar_radius: 200.0,
            polar_angle: 110.0,
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 270.0,
            color: ORBIT_3_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_3_COLOR,
            radius: 35.0,
            polar_radius: 270.0,
            polar_angle: 350.0,
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 340.0,
            color: ORBIT_4_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_4_COLOR,
            radius: 25.0,
            polar_radius: 340.0,
            polar_angle: 279.0,
        }));

        let main_window = app
            .new_window()
            .title(app.exe_name().unwrap())
            .view(view_art)
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()
            .unwrap();

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

        Model {
            main_window,
            assets,
            control_panel,
            control_panel_widget_ids,
            asset_metrics
        }
    }
}
