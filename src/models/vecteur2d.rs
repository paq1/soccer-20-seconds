#[derive(Clone)]
pub struct Vecteur2D {
    pub x: f32,
    pub y: f32
}

impl Vecteur2D {

    pub fn norme(&self) -> f32 {
        f32::sqrt(f32::powf(self.x, 2.0) + f32::powf(self.y, 2.0))
    }

    pub fn unitaire(&self) -> Self {
        Self {
            x: self.x / self.norme(),
            y: self.y / self.norme()
        }
    }

    pub fn from(position: (f32, f32), angle: f32) -> Self {

        let radiant = angle * std::f32::consts::PI / 180.0;

        let norme = Vecteur2D {
            x: position.0,
            y: position.1
        }.norme();

        let direction = Vecteur2D {
            x: norme * f32::cos(radiant),
            y: norme * f32::sin(radiant)
        };

        direction.unitaire()
    }
}