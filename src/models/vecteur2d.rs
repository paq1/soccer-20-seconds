pub struct Vecteur2D {
    pub x: f32,
    pub y: f32
}

impl Vecteur2D {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x: x2 - x1,
            y: y2 - y1
        }
    }

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

        let direction = Vecteur2D {
            x: position.0 * f32::cos(radiant),
            y: position.1 * f32::sin(radiant)
        };

        direction.unitaire()
    }
}