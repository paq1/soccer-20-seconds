use ggez::{GameResult, graphics};
use ggez::graphics::{DrawParam, Text, Transform};
use ggez::mint::{Point2, Vector2};
use crate::models::Input;
use crate::states::ingame::State;

pub struct Menu {
    image_ballon: graphics::Image,
    angle: f32,
}

impl Menu {

    pub fn load(_ctx: &mut ggez::Context) -> GameResult<Self> {
        Ok(Self {
            image_ballon: graphics::Image::from_path(_ctx, "/menu_ballon.png")?,
            angle: 0.0
        })
    }

    pub fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<State> {
        let k_ctx = &_ctx.keyboard;
        let keys = k_ctx
            .pressed_keys()
            .iter()
            .filter_map(|keycode| Input::from_keycode(keycode))
            .collect::<Vec<Input>>();

        self.angle += 100.0 * _ctx.time.delta().as_secs_f32();

        if self.angle > 360.0 {
            self.angle -= 360.0;
        }

        if keys.contains(&Input::SPACE) {
            Ok(State::Game)
        } else {
            Ok(State::Menu)
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0])
        );

        let press_to_start_text = Text::new(format!("press [SPACE] to start"));

        let win_width = ctx.gfx.size().0;
        let win_height = ctx.gfx.size().1;

        canvas.draw(
            &self.image_ballon,
            DrawParam {
                transform: Transform::Values {
                    dest: Point2 {x: win_width / 2.0, y: win_height / 2.0},
                    rotation: self.angle * std::f32::consts::PI / 180.0,
                    scale: Vector2 {x: 1.0, y: 1.2},
                    offset: Point2 {x: 0.5, y: 0.5}, // on centre le texte
                },
                ..Default::default()
            }
        );

        canvas.draw(
            &press_to_start_text,
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