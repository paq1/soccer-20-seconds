use crate::models::vecteur2d::Vecteur2D;

#[derive(Eq, PartialEq, Hash)]
pub enum Tile {
    HerbeClaire,
    HerbeFoncee,
    Mur
}

pub struct TileLayout {
    pub tiles: Vec<Vec<(Tile, Vecteur2D)>>,
}

pub struct Tilemap {
    pub layouts: Vec<TileLayout>,
    pub tile_size: u32,
    pub size: (u32, u32)
}

impl Tilemap {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            layouts: vec![
                Self::create_grass_layout(&width.unwrap_or(100), &height.unwrap_or(50))
            ],
            tile_size: 32,
            size: (width.unwrap_or(100), height.unwrap_or(50))
        }
    }

    fn create_grass_layout(width: &u32, height: &u32) -> TileLayout {
        let layout_vec = (0..*height)
            .map(|line| {
                (0..*width)
                    .map(|column| {
                        let position = Vecteur2D { x: column as f32 * 32.0, y: line as f32 * 32.0 };

                        if line == 0 || column == 0 || line == height - 1 || column == width - 1 {
                            (Tile::Mur, position)
                        } else if line % 2 == 0 {
                            (Tile::HerbeFoncee, position)
                        } else {
                            (Tile::HerbeClaire, position)
                        }
                    })
                    .collect::<Vec<(Tile, Vecteur2D)>>()
                })
            .collect::<Vec<Vec<(Tile, Vecteur2D)>>>();
        TileLayout {
            tiles: layout_vec
        }
    }

    /*
    fn create_collide_layout(width: &u32, height: &u32) -> TileLayout {
        let layout_vec = (0..*height)
            .map(|line| {
                (0..*width)
                    .map(|column| {
                        let position = Vecteur2D { x: column as f32 * 32.0, y: line as f32 * 32.0 };
                        if line == 0 || line == height - 1 || column == 0 || column == width - 1 {
                            (Tile::Mur, position)
                        } else {
                            (Tile::Empty, position)
                        }
                    })
                    .collect::<Vec<(Tile, Vecteur2D)>>()
            })
            .collect::<Vec<Vec<(Tile, Vecteur2D)>>>();
        TileLayout {
            tiles: layout_vec
        }
    }
     */

    pub fn is_in(&self, position: &Vecteur2D) -> bool {
        position.x >= 0.0 && position.x <= self.size.0 as f32 * self.tile_size as f32 &&
        position.y >= 0.0 && position.y <= self.size.1 as f32 * self.tile_size as f32
    }
}

impl TileLayout {
    pub fn collide_with(&self, position: (f32, f32), tile_size: f32) -> bool {
        let c = position.0 / tile_size;
        let l = position.1 / tile_size;
        let centre = &self.tiles[l as usize][c as usize].0;
        *centre == Tile::Mur
    }
}