pub mod vecteur2d;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Input {
    UP,
    RIGHT,
    DOWN,
    LEFT,
    SPACE,
}