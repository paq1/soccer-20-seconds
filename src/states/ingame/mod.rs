
use ggez::{event, GameResult};

use crate::states::ingame::game::Game;
use crate::states::ingame::menu::Menu;


mod player;
mod tilemap;
mod ballon;
mod but;
mod gardien;
mod game;
mod menu;

#[derive(Debug)]
pub enum State {
    Menu,
    Game
}

pub struct InGame {
    game: Game,
    menu: Menu,
    state: State,
}

impl InGame {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {

        let state = InGame {
            game: Game::load(ctx)?,
            menu: Menu::load(ctx)?,
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
            }
        }
    }
}