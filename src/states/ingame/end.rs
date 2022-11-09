use ggez::{GameResult, graphics};
use ggez::graphics::{DrawParam, Text, Transform};
use ggez::mint::{Point2, Vector2};
use crate::models::Input;
use crate::states::ingame::State;

pub struct End {
    pub restart: bool
}

impl End {

    pub fn load(_ctx: &mut ggez::Context) -> GameResult<Self> {
        Ok(Self {
            restart: false
        })
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<State> {
        let k_ctx = &_ctx.keyboard;
        let keys = k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>();

        if keys.contains(&Input::R) {
            self.restart = true;
        }
        Ok(State::End)

    }

    fn str_resultat(score: u32) -> String {
        if score < 2 {
            "You lose".to_string()
        } else if score == 2 {
            "Draw".to_string()
        } else {
            "You win".to_string()
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context, score: u32) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );

        let score_text = Text::new(format!("score : {} - 2", score));
        let mut resultat_text = Text::new(format!("{}", End::str_resultat(score)));
        resultat_text.set_scale(32.0);
        let restart_text = Text::new(format!("press [R] to restart"));


        let win_width = ctx.gfx.size().0;
        let win_height = ctx.gfx.size().1;

        canvas.draw(
            &resultat_text,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: win_width / 2.0, y: win_height / 2.0 - 100.0},
                    rotation: 0.0,
                    scale: Vector2 {x: 1.0, y: 1.0},
                    offset: Point2 {x: 0.5, y: 0.5}, // on centre le texte
                },
                ..Default::default()
            }
        );

        canvas.draw(
            &score_text,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: win_width / 2.0, y: win_height / 2.0},
                    rotation: 0.0,
                    scale: Vector2 {x: 1.0, y: 1.0},
                    offset: Point2 {x: 0.5, y: 0.5}, // on centre le texte
                },
                ..Default::default()
            }
        );

        canvas.draw(
            &restart_text,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: win_width / 2.0, y: win_height / 2.0 + 100.0},
                    rotation: 0.0,
                    scale: Vector2 {x: 1.0, y: 1.0},
                    offset: Point2 {x: 0.5, y: 0.5}, // on centre le texte
                },
                ..Default::default()
            }
        );

        canvas.finish(ctx)
    }
}