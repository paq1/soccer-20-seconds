
use ggez::{event, GameResult};

use crate::states::ingame::game::Game;
use crate::states::ingame::menu::Menu;
use crate::states::ingame::end::End;


mod player;
mod tilemap;
mod ballon;
mod but;
mod gardien;
mod game;
mod menu;
mod end;

#[derive(Debug)]
pub enum State {
    Menu,
    Game,
    End
}

pub struct InGame {
    game: Game,
    menu: Menu,
    end: End,
    state: State,
}

impl InGame {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {

        let state = InGame {
            game: Game::load(ctx)?,
            menu: Menu::load(ctx)?,
            end: End::load(ctx)?,
            state: State::Menu,
        };

        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for InGame {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {

        match self.state {
            State::Game => {
                self.state = self.game.update(_ctx).unwrap();
                Ok(())
            },
            State::Menu => {
                self.state = self.menu.update(_ctx).unwrap();
                Ok(())
            },
            State::End => {
                self.state = self.end.update(_ctx).unwrap();

                if self.end.restart {
                    self.game = Game::load(_ctx).unwrap();
                    self.state = State::Menu;
                }

                Ok(())
            }
        }
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {

        match self.state {
            State::Game => {
                self.game.draw(ctx)
            },
            State::Menu => {
                self.menu.draw(ctx)
            },
            State::End => {
                self.end.draw(ctx, self.game.score)
            }
        }
    }
}