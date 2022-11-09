use crate::models::vecteur2d::Vecteur2D;
use crate::states::ingame::ballon::Ballon;

pub struct Gardien {
    pub position: Vecteur2D,
    pub targets: Vec<Vecteur2D>,
    pub index_current_target: u32,
}

impl Gardien {

    pub fn update_position(&mut self, dt: f32) {
        let target = self.targets.get(self.index_current_target as usize).unwrap();

        let v_target_gardien = Vecteur2D {
            x: target.x - self.position.x,
            y: target.y - self.position.y,
        };

        let distance_target_gardien = v_target_gardien.norme();

        if distance_target_gardien < 5.0 {
            let new_index = (self.index_current_target + 1) % (self.targets.len() as u32);
            self.index_current_target = new_index;
        }

        let new_target = self.targets.get(self.index_current_target as usize).unwrap();
        let v_target_gardien = Vecteur2D {
            x: new_target.x - self.position.x,
            y: new_target.y - self.position.y,
        };

        let v_target_gardien_unitaire = v_target_gardien.unitaire();

        self.position.x += v_target_gardien_unitaire.x * 100.0 * dt;
        self.position.y += v_target_gardien_unitaire.y * 100.0 * dt;

        println!("{}", v_target_gardien_unitaire.y * 100.0);
    }

    pub fn catch_the_ball(&mut self, ballon: &Ballon) -> bool {
        let vecteur_ballon_gardien = Vecteur2D {
            x: ballon.position.x - self.position.x,
            y: ballon.position.y - self.position.y,
        };

        let distance = vecteur_ballon_gardien.norme();

        if distance < 16.0 {
            println!("Gardien a attrapé le ballon");
        }

        distance < 16.0
    }
}