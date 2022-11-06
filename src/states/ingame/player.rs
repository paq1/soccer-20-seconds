use crate::models::Input;
use crate::states::ingame::tilemap::TileLayout;

const DEFAULT_POSITION: (f32, f32) = (0.0, 0.0);
const VITESSE: f32 = 200.0;

pub struct Player {
    pub position: (f32, f32),
    pub angle: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            position: DEFAULT_POSITION,
            angle: -90.0,
        }
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            position,
            angle: -90.0,
        }
    }

    pub fn update_deplacement(
        &mut self, keys: Vec<Input>,
        collide_layout: &TileLayout,
        dt: f32
    ) {

        let old_position = self.position;

        keys.iter()
            .for_each(|key| {
                match key {
                    Input::UP => {
                        // self.position.1 -= VITESSE * dt

                    },
                    Input::RIGHT => {
                        // self.position.0 += VITESSE * dt;
                        self.angle += VITESSE * dt;
                    },
                    Input::DOWN => self.position.1 += VITESSE * dt,
                    Input::LEFT => {
                        // self.position.0 -= VITESSE * dt;
                        self.angle -= VITESSE * dt;
                    },
                }
            });

        if collide_layout.collide_with(self.position, 32.0) {
            self.position = old_position;
        }

    }
}