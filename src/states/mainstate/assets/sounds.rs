use ggez::{audio, GameResult};

pub struct GameSounds {
    pub son_shoot: audio::Source,
    pub son_siiuu: audio::Source,
    pub son_bruh: audio::Source,
}

impl GameSounds {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let sounds = GameSounds {
            son_shoot: audio::Source::new(ctx, "/sounds/shoot.wav")?,
            son_siiuu: audio::Source::new(ctx, "/sounds/siiuu.mp3")?,
            son_bruh: audio::Source::new(ctx, "/sounds/bruh.mp3")?,
        };
        Ok(sounds)
    }
}