use crate::models::vecteur2d::Vecteur2D;

pub struct But {
    pub position: Vecteur2D,
    pub size: Vecteur2D,
}

impl But {
    pub fn is_in(&self, position: &Vecteur2D) -> bool {

        let is_in_x = position.x >= self.position.x && position.x <= self.position.x + self.size.x;
        let is_in_y = position.y >= self.position.y && position.y <= self.position.y + self.size.y;

        is_in_x && is_in_y
    }
}