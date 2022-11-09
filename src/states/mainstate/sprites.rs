use std::collections::HashMap;
use ggez::{GameResult, graphics};
use crate::states::mainstate::tilemap::Tile;

pub struct GameSprites {
    pub player_image: graphics::Image,
    pub goal_image: graphics::Image,
    pub tiles_images: HashMap<Tile, graphics::Image>,
    pub ballon_image: graphics::Image,
    pub but_image: graphics::Image,
    pub key_arrow_image: graphics::Image,
    pub key_arrow_pressed_image: graphics::Image,
    pub key_space_image: graphics::Image,
    pub key_space_pressed_image: graphics::Image,
}

impl GameSprites {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let sprites = Self {
            player_image: graphics::Image::from_path(ctx, "/player-calvitie.png")?,
            goal_image: graphics::Image::from_path(ctx, "/player-roux.png")?,
            tiles_images: HashMap::from([
                (Tile::HerbeClaire, graphics::Image::from_path(ctx, "/tiles/tile-herbe-claire.png")?),
                (Tile::HerbeFoncee, graphics::Image::from_path(ctx, "/tiles/tile-herbe-foncee.png")?),
                (Tile::Mur, graphics::Image::from_path(ctx, "/tiles/brique.png")?),
            ]),
            ballon_image: graphics::Image::from_path(ctx, "/ballon.png")?,
            but_image: graphics::Image::from_path(ctx, "/goal-left.png")?,
            key_arrow_image: graphics::Image::from_path(ctx, "/key-arrow.png")?,
            key_arrow_pressed_image: graphics::Image::from_path(ctx, "/key-arrow-pressed.png")?,
            key_space_image: graphics::Image::from_path(ctx, "/key-space.png")?,
            key_space_pressed_image: graphics::Image::from_path(ctx, "/key-space-pressed.png")?,
        };
        Ok(sprites)
    }
}

pub struct MenuSprites {
    pub image_ballon: graphics::Image,
    pub image_titre: graphics::Image,
}

impl MenuSprites {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let sprites = Self {
            image_ballon: graphics::Image::from_path(ctx, "/menu_ballon.png")?,
            image_titre: graphics::Image::from_path(ctx, "/titre.png")?,
        };
        Ok(sprites)
    }
}