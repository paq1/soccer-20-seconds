use crate::models::Input;
use crate::models::vecteur2d::Vecteur2D;
use crate::states::mainstate::game_element::tilemap::TileLayout;

const DEFAULT_POSITION: (f32, f32) = (0.0, 0.0);
const VITESSE: f32 = 200.0;
const PUISSANCE_SHOOT: f32 = 500.0;

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
            angle: 180.0,
        }
    }

    pub fn update_deplacement(
        &mut self, keys: &Vec<Input>,
        collide_layout: &TileLayout,
        tile_size: f32,
        dt: f32
    ) {

        let old_position = self.position;

        keys.iter()
            .for_each(|key| {
                match key {
                    Input::RIGHT => {
                        self.angle += VITESSE * dt;
                    },
                    Input::LEFT => {
                        self.angle -= VITESSE * dt;
                    },
                    _ => {}
                }
            });

        self.angle = self.update_angle();

        keys.iter()
            .for_each(|key| {
                match key {
                    Input::UP => {
                        let direction = Vecteur2D::from(self.position, self.angle);

                        self.position.0 += direction.x * VITESSE * dt;
                        self.position.1 += direction.y * VITESSE * dt;
                    },
                    Input::DOWN => {
                        let direction = Vecteur2D::from(self.position, self.angle);

                        self.position.0 -= direction.x * VITESSE * dt;
                        self.position.1 -= direction.y * VITESSE * dt;
                    },
                    _ => {}
                }
            });

        if collide_layout.collide_with(self.position, tile_size) {
            self.position = old_position;
        }

    }

    fn update_angle(&self) -> f32 {
        if self.angle > 360.0 {
            self.angle - 360.0
        } else if self.angle < 0.0 {
            self.angle + 360.0
        } else {
            self.angle
        }
    }

    fn get_direction(&self) -> Vecteur2D {
        Vecteur2D::from(self.position, self.angle)
    }

    pub fn shoot(&self) -> Vecteur2D {
        let direction = self.get_direction();

        Vecteur2D {
            x: direction.x * PUISSANCE_SHOOT,
            y: direction.y * PUISSANCE_SHOOT
        }
    }

}