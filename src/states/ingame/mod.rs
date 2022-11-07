use std::collections::HashMap;

use ggez::{event, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Transform};
use ggez::mint::{Point2, Vector2};

use player::Player;
use tilemap::Tilemap;

use crate::models::Input;
use crate::models::vecteur2d::Vecteur2D;
use crate::states::ingame::ballon::Ballon;
use crate::states::ingame::tilemap::Tile;

mod player;
mod tilemap;
mod ballon;

pub struct InGame {
    show_debug: bool,
    player: Player,
    tilemap: Tilemap,
    ballon: Ballon,
    player_image: graphics::Image,
    tiles_images: HashMap<Tile, graphics::Image>,
    ballon_image: graphics::Image,
    dt: f32
}

impl InGame {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<InGame> {
        let state = InGame {
            show_debug: true,
            player: Player :: new((100.0, 100.0)),
            tilemap: Tilemap::new(Some(25), Some(13)), // 25 / 13
            ballon: Ballon { position: Vecteur2D { x: 100.0, y: 100.0}, direction_du_shoot: None },
            player_image: graphics::Image::from_path(ctx, "/player-calvitie.png")?,
            tiles_images: HashMap::from([
                (Tile::HerbeClaire, graphics::Image::from_path(ctx, "/tiles/tile-herbe-claire.png")?),
                (Tile::HerbeFoncee, graphics::Image::from_path(ctx, "/tiles/tile-herbe-foncee.png")?),
                (Tile::Mur, graphics::Image::from_path(ctx, "/tiles/brique.png")?),
            ]),
            ballon_image: graphics::Image::from_path(ctx, "/ballon.png")?,
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

        self.player.update_deplacement(keys, self.tilemap.layouts.get(0).unwrap(), self.tilemap.tile_size as f32, self.dt);
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
        self.ballon.update_position(self.player.position, self.player.angle);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text = ggez::graphics::Text::new(format!("fps ({})", ctx.time.fps()));
        let text_dt = ggez::graphics::Text::new(format!("dt ({})", self.dt));

        let win_width = ctx.gfx.size().0;
        let win_height = ctx.gfx.size().1;

        // todo mettre a jour si notion de camera
        self.tilemap.layouts
            .iter()
            .for_each(|layout| {
                let nb_l_layout = layout.tiles.len();
                let nb_c_layout = match layout.tiles.get(0) {
                    Some(colonne) => colonne.len(),
                    None => 0
                };

                let max_index_width = win_width / self.tilemap.tile_size as f32;
                let max_index_height = win_height / self.tilemap.tile_size as f32;

                let index_max_c = if nb_c_layout > max_index_width as usize {
                    max_index_width as usize
                } else {
                    nb_c_layout
                };

                let index_max_l = if nb_l_layout > max_index_height as usize {
                    max_index_height as usize
                } else {
                    nb_l_layout
                };

                for l in 0..index_max_l {
                    for c in 0..index_max_c {
                        let tile = &layout.tiles[l][c].0;
                        match self.tiles_images.get(tile) {
                            Some(image) => {
                                let tile_position = Point2 {
                                    x: c as f32 * self.tilemap.tile_size as f32,
                                    y: l as f32 * self.tilemap.tile_size as f32
                                };
                                // canvas.draw(image, tile_position);

                                canvas.draw(
                                    image,
                                    DrawParam {
                                        transform: Transform::Values {
                                            dest: tile_position,
                                            rotation: 0.0,
                                            scale: Vector2 {x: 1.0, y: 1.0},
                                            offset: Point2 {x: 0.0, y: 0.0},
                                        },
                                        ..Default::default()
                                    }
                                )
                            },
                            None => {}
                        }
                    }
                }

            });

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

            canvas.draw(
                &self.ballon_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: self.ballon.position.x, y: self.ballon.position.y},
                        rotation: 0.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            )
        }

        canvas.finish(ctx)
    }
}