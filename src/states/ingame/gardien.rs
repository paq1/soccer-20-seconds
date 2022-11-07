use crate::models::vecteur2d::Vecteur2D;

pub struct Gardien {
    pub position: Vecteur2D,
    pub targets: Vec<Vecteur2D>,
    pub index_current_target: u32,
}

impl Gardien {

    pub fn update_position(&mut self, dt: f32) {
        let target = self.targets.get(self.index_current_target as usize).unwrap();

        let v = Vecteur2D {
            x: target.x - self.position.x,
            y: target.y - self.position.y,
        };

        let distance = v.norme();

        if distance < 1.0 {
            let new_index = self.index_current_target + 1;
            if new_index >= self.targets.len() as u32 {
                self.index_current_target = 0;
            } else {
                self.index_current_target = new_index;
            }
        }

        let v_unitaire = v.unitaire();

        self.position.x += v_unitaire.x * 100.0 * dt;
        self.position.y += v_unitaire.y * 100.0 * dt;

        println!("Gardien position: {:?}", self.position);
    }
}