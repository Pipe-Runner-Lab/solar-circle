use crate::prelude::*;

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

        // TODO: Read life cycle
        let (control_panel_widget_ids, control_panel) = control_panel_setup(app);

        Model {
            main_window,
            assets,
            control_panel,
            control_panel_widget_ids,
            asset_metrics,
        }
    }
}
