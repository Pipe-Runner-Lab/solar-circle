use crate::prelude::*;

pub struct Model {
    pub window: window::Id,
    pub assets: Vec<ASSETS>,
}

impl Model {
    /// This function creates a new model instance
    pub fn new(app: &App) -> Self {
        let mut assets = Vec::new();

        assets.push(ASSETS::SUN(Sun {}));
        assets.push(ASSETS::ORBIT(Orbit {
            radius: 130.0,
            color: ORBIT_1_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_1_COLOR,
            radius: 20.0,
            polar_radius: 130.0,
            polar_angle: 45.0
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 200.0,
            color: ORBIT_2_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_2_COLOR,
            radius: 10.0,
            polar_radius: 200.0,
            polar_angle: 110.0
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 270.0,
            color: ORBIT_3_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_3_COLOR,
            radius: 35.0,
            polar_radius: 270.0,
            polar_angle: 350.0
        }));

        assets.push(ASSETS::ORBIT(Orbit {
            radius: 340.0,
            color: ORBIT_4_COLOR,
        }));
        assets.push(ASSETS::PLANET(Planet {
            color: ORBIT_4_COLOR,
            radius: 25.0,
            polar_radius: 340.0,
            polar_angle: 279.0
        }));

        let window = app
            .new_window()
            .view(view_art)
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()
            .unwrap();
        Model { window, assets }
    }
}
