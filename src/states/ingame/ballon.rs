use crate::models::vecteur2d::Vecteur2D;

#[derive(Clone)]
pub struct Ballon {
    pub position: Vecteur2D,
    pub direction_du_shoot: Option<Vecteur2D>,
}

impl Ballon {
    pub fn update_position(&self, position_joueur: (f32, f32), angle_joueur: f32, dt: f32) -> Self {

        match &self.direction_du_shoot {
            Some(direction) => {
                let new_position = Vecteur2D {
                    x: self.position.x + direction.x * dt,
                    y: self.position.y + direction.y * dt,
                };

                Ballon {
                    position: new_position,
                    direction_du_shoot: Some(direction.clone()),
                }
            },
            None => {
                let direction = Vecteur2D::from(position_joueur, angle_joueur);
                let position_ballon = Vecteur2D {
                    x: position_joueur.0 + direction.x * 32.0,
                    y: position_joueur.1 + direction.y * 32.0
                };
                Ballon {
                    position: position_ballon,
                    direction_du_shoot: None,
                }
            }
        }
    }
}