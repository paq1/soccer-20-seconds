use std::collections::HashMap;
use std::f32::consts::PI;

use ggez::{event, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Transform};
use ggez::mint::{Point2, Vector2};

use player::Player;
use tilemap::Tilemap;

use crate::models::Input;
use crate::states::ingame::tilemap::Tile;

mod player;
mod tilemap;

pub struct InGame {
    show_debug: bool,
    player: Player,
    tilemap: Tilemap,
    player_image: graphics::Image,
    tiles_images: HashMap<Tile, graphics::Image>,
    dt: f32
}

impl InGame {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<InGame> {
        let state = InGame {
            show_debug: true,
            player: Player :: new((100.0, 100.0)),
            tilemap: Tilemap::new(Some(25), Some(13)),
            player_image: graphics::Image::from_path(ctx, "/player-calvitie.png")?,
            tiles_images: HashMap::from([
                (Tile::HerbeClaire, graphics::Image::from_path(ctx, "/tiles/tile-herbe-claire.png")?),
                (Tile::HerbeFoncee, graphics::Image::from_path(ctx, "/tiles/tile-herbe-foncee.png")?),
                (Tile::Mur, graphics::Image::from_path(ctx, "/tiles/brique.png")?),
            ]),
            dt: 0.0
        };
        Ok(state)
    }

    pub fn update_kb(&mut self, _ctx: &mut ggez::Context) -> GameResult {

        let k_ctx = &_ctx.keyboard;

        let keys = k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>();

        self.player.update_deplacement(keys, self.tilemap.layouts.get(1).unwrap(), self.dt);
        self.update_activate_debug(_ctx);

        Ok(())
    }

    pub fn update_dt(&mut self, ctx: &mut ggez::Context) {
        self.dt = ctx.time.delta().as_secs_f32();
    }

    pub fn update_activate_debug(&mut self, ctx: &mut ggez::Context) {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::F1) {
            self.show_debug = !self.show_debug;
        }
    }
}

impl event::EventHandler<ggez::GameError> for InGame {

    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        self.update_dt(_ctx);
        self.update_kb(_ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text = ggez::graphics::Text::new(format!("fps ({})", ctx.time.fps()));
        let text_dt = ggez::graphics::Text::new(format!("dt ({})", self.dt));

        self.tilemap.layouts
            .iter()
            .for_each(|layout| {
                layout.tiles
                    .iter()
                    .for_each(|tiles| {
                        tiles
                            .iter()
                            .filter(|tile| tile.0 != Tile::Empty)
                            .for_each(|tile| {
                                let tile_image = self.tiles_images.get(&tile.0).unwrap();
                                canvas.draw(tile_image, Vec2 { x: tile.1.x, y: tile.1.y })
                            });
                    });
            });

        /*
        self.tilemap.layouts
            .iter()
            .for_each(|layout| {
                layout.tiles
                    .iter()
                    .enumerate()
                    .for_each(|(ligne, tiles)| {
                        tiles
                            .iter()
                            .enumerate()
                            .for_each(|(colonne, tile)| {
                                let x = colonne as f32 * self.tilemap.tile_size as f32;
                                let y = ligne as f32 * self.tilemap.tile_size as f32;

                                let tile_image = self.tiles_images.get(tile);
                                match tile_image {
                                    Some(image) => canvas.draw(image, Vec2 { x, y }),
                                    None => ()
                                }
                            });
                    });
            });

         */

        canvas.draw(
            &self.player_image,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: self.player.position.0, y: self.player.position.1},
                    rotation: (self.player.angle + 90.0) * std::f32::consts::PI / 180.0,
                    scale: Vector2 {x: 1.0, y: 1.0},
                    offset: Point2 {x: 0.5, y: 0.5},
                },
                ..Default::default()
                }
        );

        if self.show_debug {
            canvas.draw(&text, Vec2 { x: 0.0, y: 0.0 });
            canvas.draw(&text_dt, Vec2 { x: 0.0, y: 32.0 });
        }

        canvas.finish(ctx)
    }
}