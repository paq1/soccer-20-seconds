use std::collections::HashMap;
use ggez::{GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{DrawParam, Text, Transform};
use ggez::mint::{Point2, Vector2};
use crate::models::Input;
use crate::models::vecteur2d::Vecteur2D;
use crate::states::ingame::ballon::Ballon;
use crate::states::ingame::but::But;
use crate::states::ingame::gardien::Gardien;
use crate::states::ingame::player::Player;
use crate::states::ingame::State;
use crate::states::ingame::tilemap::{Tile, Tilemap};

pub struct Game {
    show_debug: bool,
    player: Player,
    gardien: Gardien,
    tilemap: Tilemap,
    ballon: Option<Ballon>,
    but: But,
    score: u32,
    player_image: graphics::Image,
    goal_image: graphics::Image,
    tiles_images: HashMap<Tile, graphics::Image>,
    ballon_image: graphics::Image,
    but_image: graphics::Image,
    timer: f32,
    dt: f32
}

impl Game {
    pub fn load(ctx: &mut ggez::Context) -> GameResult<Game> {
        let gardien_position = Vecteur2D {x: 32.0 * 2.0 + 16.0, y: 32.0 * 6.0 + 16.0};
        Ok(Self {
            show_debug: true,
            player: Player :: new((32.0 * 16.0 + 16.0, 32.0 * 6.0 + 16.0)),
            gardien: Gardien {
                position: gardien_position.clone(),
                targets: vec![
                    Vecteur2D {x: gardien_position.x, y: gardien_position.y + 32.0 * 1.0},
                    Vecteur2D {x: gardien_position.x, y: gardien_position.y - 32.0 * 1.0},
                ],
                index_current_target: 0,
            },
            tilemap: Tilemap::new(Some(25), Some(13)), // 25 / 13
            ballon: Some(Ballon { position: Game::position_centre( Vecteur2D {x: 25.0, y: 13.0}, 32.0 ), direction_du_shoot: None, est_au_centre: true }),
            but: But { position: Vecteur2D { x: 32.0, y: 5.0 * 32.0}, size: Vecteur2D { x: 32.0, y: 32.0 * 3.0} },
            score: 0,
            player_image: graphics::Image::from_path(ctx, "/player-calvitie.png")?,
            goal_image: graphics::Image::from_path(ctx, "/player-roux.png")?,
            tiles_images: HashMap::from([
                (Tile::HerbeClaire, graphics::Image::from_path(ctx, "/tiles/tile-herbe-claire.png")?),
                (Tile::HerbeFoncee, graphics::Image::from_path(ctx, "/tiles/tile-herbe-foncee.png")?),
                (Tile::Mur, graphics::Image::from_path(ctx, "/tiles/brique.png")?),
            ]),
            ballon_image: graphics::Image::from_path(ctx, "/ballon.png")?,
            but_image: graphics::Image::from_path(ctx, "/goal-left.png")?,
            timer: 20.0,
            dt: 0.0
        })
    }

    fn position_centre(size_tilemap: Vecteur2D, tile_size: f32) -> Vecteur2D {
        let x = size_tilemap.x * tile_size / 2.0;
        let y = size_tilemap.y * tile_size / 2.0;
        Vecteur2D { x, y }
    }

    pub fn update_timer(&mut self) -> GameResult<State> {
        self.timer -= self.dt;
        if self.timer <= 0.0 {
            Ok(State::Menu)
        } else {
            Ok(State::Game)
        }
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<State> {
        self.update_dt(_ctx);
        self.update_kb(_ctx)?;
        self.update_ballon();
        self.gardien.update_position(self.dt);

        self.update_timer()
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text = Text::new(format!("fps ({})", ctx.time.fps()));
        let text_dt = Text::new(format!("dt ({})", self.dt));
        let text_score = Text::new(format!("fra {} - 0 por", self.score));
        let text_temps = Text::new(format!("temps {}", self.timer as u32));

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

        canvas.draw(
            &self.goal_image,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: self.gardien.position.x, y: self.gardien.position.y},
                    rotation: 90.0 * std::f32::consts::PI / 180.0,
                    scale: Vector2 {x: 1.0, y: 1.0},
                    offset: Point2 {x: 0.5, y: 0.5},
                },
                ..Default::default()
            }
        );

        self.ballon
            .as_ref()
            .map(|ballon| {
                canvas.draw(
                    &self.ballon_image,
                    DrawParam {
                        transform: Transform::Values {
                            dest: Point2 {x: ballon.position.x, y: ballon.position.y},
                            rotation: 0.0,
                            scale: Vector2 {x: 1.0, y: 1.0},
                            offset: Point2 {x: 0.5, y: 0.5},
                        },
                        ..Default::default()
                    }
                )
            });

        canvas.draw(
            &self.but_image,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: self.but.position.x, y: self.but.position.y},
                    rotation: 0.0,
                    scale: Vector2 {x: 2.0, y: 3.0},
                    offset: Point2 {x: 0.0, y: 0.0},
                },
                ..Default::default()
            }
        );

        canvas.draw(&text_score, Vec2 {x: 32.0 * 11.0, y: 0.0});
        canvas.draw(&text_temps, Vec2 {x: 32.0 * 12.0, y: 16.0});

        if self.show_debug {
            canvas.draw(&text, Vec2 { x: 0.0, y: 0.0 });
            canvas.draw(&text_dt, Vec2 { x: 0.0, y: 32.0 });
        }

        canvas.finish(ctx)
    }

    pub fn update_kb(&mut self, _ctx: &mut ggez::Context) -> GameResult {

        let k_ctx = &_ctx.keyboard;

        let keys = k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>();

        self.player.update_deplacement(&keys, self.tilemap.layouts.get(0).unwrap(), self.tilemap.tile_size as f32, self.dt);

        // on met a jour le shoot du joueur
        let shoot = self.player.shoot();
        if keys.contains(&Input::SPACE) {
            self.ballon = self.ballon
                .as_ref()
                .map(|ballon| {
                    if ballon.direction_du_shoot.is_none() && !ballon.est_au_centre {
                        Ballon {
                            position: ballon.position.clone(),
                            direction_du_shoot: Some(shoot),
                            est_au_centre: false
                        }
                    } else {
                        ballon.clone()
                    }
                });
        }

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

    pub fn update_ballon(&mut self) {
        self.ballon = match self.ballon {
            Some(ref mut ballon) => {
                // on met a jour la position du ballon
                let pos_a_jour = ballon.update_position(self.player.position, self.player.angle, self.dt);

                // on verifie si le ballon est dans but
                if self.but.is_in(&pos_a_jour.position) {
                    self.score += 1;
                    None // on supprime le ballon
                }
                // on verifie si le ballon est dans le gardien
                else if self.gardien.catch_the_ball(ballon) {
                    None // on supprime le ballon mais on ne marque pas de but

                }
                // on verifie si le ballon est en dehors du terrain
                else if !self.tilemap.is_in(&pos_a_jour.position) {
                    None // on supprime le ballon
                }

                else if pos_a_jour.est_au_centre {

                    let distance_joueur_ballon = Vecteur2D {
                        x: self.player.position.0 - pos_a_jour.position.x,
                        y: self.player.position.1 - pos_a_jour.position.y
                    }.norme();

                    if distance_joueur_ballon < 32.0 {
                        // le joueur récupère le ballon
                        Some(
                            Ballon {
                                position: pos_a_jour.position,
                                direction_du_shoot: None,
                                est_au_centre: false
                            }
                        )
                    }
                    else {
                        Some(ballon.clone())
                    }
                }
                else {
                    Some(pos_a_jour)
                }
            },
            None => {
                let position_centre = Vecteur2D {
                    x: self.tilemap.size.0 as f32 * self.tilemap.tile_size as f32 / 2.0,
                    y: self.tilemap.size.1 as f32 * self.tilemap.tile_size as f32 / 2.0
                };

                Some(Ballon { position: position_centre, direction_du_shoot: None, est_au_centre: true })
            }
        }
    }
}