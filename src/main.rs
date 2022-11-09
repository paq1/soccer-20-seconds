mod states;
mod models;
mod mappings;

use ggez::{
    event,
    GameResult,
};
use crate::states::mainstate::MainState;


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hello_world", "mkd")
        .add_resource_path("./resources");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;

    ctx.gfx.set_window_title("soccer-20-seconds -- @paq1");
    event::run(ctx, event_loop, state)
}
