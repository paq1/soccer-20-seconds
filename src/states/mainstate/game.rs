use ggez::{audio, GameResult, graphics};
use ggez::audio::SoundSource;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam, Text, Transform};
use ggez::mint::{Point2, Vector2};
use crate::models::Input;
use crate::models::vecteur2d::Vecteur2D;
use crate::states::mainstate::ballon::Ballon;
use crate::states::mainstate::but::But;
use crate::states::mainstate::gardien::Gardien;
use crate::states::mainstate::player::Player;
use crate::states::mainstate::sprites::GameSprites;
use crate::states::mainstate::State;
use crate::states::mainstate::tilemap::Tilemap;

pub struct Game {
    show_debug: bool,
    player: Player,
    gardien: Gardien,
    tilemap: Tilemap,
    ballon: Option<Ballon>,
    but: But,
    pub score: u32,
    sprites: GameSprites,
    son_shoot: audio::Source,
    son_siiuu: audio::Source,
    son_bruh: audio::Source,
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
            sprites: GameSprites::new(ctx)?,
            son_shoot: audio::Source::new(ctx, "/sounds/shoot.wav")?,
            son_siiuu: audio::Source::new(ctx, "/sounds/siiuu.mp3")?,
            son_bruh: audio::Source::new(ctx, "/sounds/bruh.mp3")?,
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
            Ok(State::End)
        } else {
            Ok(State::Game)
        }
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<State> {
        self.update_dt(_ctx);
        self.update_kb(_ctx)?;
        self.update_ballon(_ctx);
        self.gardien.update_position(self.dt);

        self.update_timer()
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );
        let text_fps = Text::new(format!("fps ({})", ctx.time.fps() as i32));
        // let text_dt = Text::new(format!("dt ({})", self.dt));
        let text_score = Text::new(format!("fra {} - 2 por", self.score));
        let text_temps = Text::new(format!("time {}", self.timer as u32));

        let win_width = ctx.gfx.size().0;
        let win_height = ctx.gfx.size().1;

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
                        match self.sprites.tiles_images.get(tile) {
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
            &self.sprites.player_image,
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
            &self.sprites.goal_image,
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
                    &self.sprites.ballon_image,
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
            &self.sprites.but_image,
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

        self.draw_keys(ctx, &mut canvas);

        if self.show_debug {
            canvas.draw(&text_fps, Vec2 { x: 0.0, y: 0.0 });
        }

        canvas.finish(ctx)
    }

    fn draw_keys(&mut self, ctx: &mut ggez::Context,canvas: &mut Canvas) {

        let touches = self.get_keys(ctx);
        // touche espace
        if touches.contains(&Input::SPACE) {
            canvas.draw(
                &self.sprites.key_space_pressed_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 400.0, y: 500.0},
                        rotation: 0.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        } else {
            canvas.draw(
                &self.sprites.key_space_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 400.0, y: 500.0},
                        rotation: 0.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        }

        // UP
        if touches.contains(&Input::UP) {
            canvas.draw(
                &self.sprites.key_arrow_pressed_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0, y: 500.0 - 32.0},
                        rotation: 0.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        } else {
            canvas.draw(
                &self.sprites.key_arrow_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0, y: 500.0 - 32.0},
                        rotation: 0.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        }

        // DOWN
        if touches.contains(&Input::DOWN) {
            canvas.draw(
                &self.sprites.key_arrow_pressed_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0, y: 500.0 + 2.0},
                        rotation: 180.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        } else {
            canvas.draw(
                &self.sprites.key_arrow_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0, y: 500.0 + 2.0},
                        rotation: 180.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        }

        // RIGHT
        if touches.contains(&Input::RIGHT) {
            canvas.draw(
                &self.sprites.key_arrow_pressed_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0 + 34.0, y: 500.0 + 2.0},
                        rotation: 90.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        } else {
            canvas.draw(
                &self.sprites.key_arrow_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0 + 34.0, y: 500.0 + 2.0},
                        rotation: 90.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        }

        // LEFT
        if touches.contains(&Input::LEFT) {
            canvas.draw(
                &self.sprites.key_arrow_pressed_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0 - 34.0, y: 500.0 + 2.0},
                        rotation: -90.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        } else {
            canvas.draw(
                &self.sprites.key_arrow_image,
                DrawParam {
                    transform: Transform::Values {
                        dest: Point2 {x: 700.0 - 34.0, y: 500.0 + 2.0},
                        rotation: -90.0 * std::f32::consts::PI / 180.0,
                        scale: Vector2 {x: 1.0, y: 1.0},
                        offset: Point2 {x: 0.5, y: 0.5},
                    },
                    ..Default::default()
                }
            );
        }


    }

    pub fn update_kb(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        let keys = self.get_keys(_ctx);

        self.player.update_deplacement(&keys, self.tilemap.layouts.get(0).unwrap(), self.tilemap.tile_size as f32, self.dt);

        // on met a jour le shoot du joueur
        let shoot = self.player.shoot();
        if keys.contains(&Input::SPACE) {
            self.ballon = self.ballon
                .as_ref()
                .map(|ballon| {
                    if ballon.direction_du_shoot.is_none() && !ballon.est_au_centre {
                        self.son_shoot.play(_ctx).unwrap();
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

    fn get_keys(&self, _ctx: &mut ggez::Context) -> Vec<Input> {
        let k_ctx = &_ctx.keyboard;

        k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>()
    }

    pub fn update_dt(&mut self, ctx: &mut ggez::Context) {
        self.dt = ctx.time.delta().as_secs_f32();
    }

    pub fn update_activate_debug(&mut self, ctx: &mut ggez::Context) {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::F1) {
            self.show_debug = !self.show_debug;
        }
    }

    pub fn update_ballon(&mut self, ctx: &mut ggez::Context) {
        self.ballon = match self.ballon {
            Some(ref mut ballon) => {
                // on met a jour la position du ballon
                let pos_a_jour = ballon.update_position(self.player.position, self.player.angle, self.dt);

                // on verifie si le ballon est dans but
                if self.but.is_in(&pos_a_jour.position) {
                    self.son_siiuu.play(ctx).unwrap();
                    self.score += 1;
                    None // on supprime le ballon
                }
                // on verifie si le ballon est dans le gardien
                else if self.gardien.catch_the_ball(ballon) {
                    self.son_bruh.play(ctx).unwrap();

                    None // on supprime le ballon mais on ne marque pas de but
                }
                // on verifie si le ballon est en dehors du terrain
                else if !self.tilemap.is_in(&pos_a_jour.position) {
                    self.son_bruh.play(ctx).unwrap();
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