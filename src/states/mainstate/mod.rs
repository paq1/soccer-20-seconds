
use ggez::{event, GameResult};

use crate::states::mainstate::game::Game;
use crate::states::mainstate::menu::Menu;
use crate::states::mainstate::end::End;


mod game;
mod menu;
mod end;
mod assets;
mod game_element;

#[derive(Debug)]
pub enum State {
    Menu,
    Game,
    End
}

pub struct MainState {
    game: Game,
    menu: Menu,
    end: End,
    state: State,
}

impl MainState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {

        let state = MainState {
            game: Game::load(ctx)?,
            menu: Menu::load(ctx)?,
            end: End::load(ctx)?,
            state: State::Menu,
        };

        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
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
                    self.end.restart = false;
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