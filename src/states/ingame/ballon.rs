use crate::models::vecteur2d::Vecteur2D;

pub struct Ballon {
    pub position: Vecteur2D,
    pub direction_du_shoot: Option<Vecteur2D>,
}

impl Ballon {
    pub fn update_position(&mut self, position_joueur: (f32, f32), angle_joueur: f32) {
        if self.direction_du_shoot.is_none() {
            let direction = Vecteur2D::from(position_joueur, angle_joueur);
            let position_ballon = Vecteur2D {
                x: position_joueur.0 + direction.x * 32.0,
                y: position_joueur.1 + direction.y * 32.0
            };
            self.position = position_ballon;
        }
    }
}