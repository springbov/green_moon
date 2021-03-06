
use game::Game;

pub trait Scene {
    fn enter(&mut self, game: &mut Game) -> bool { false }

    fn leave(&mut self, game: &mut Game) -> bool { false }

    fn update(&mut self, game: &mut Game) -> bool { false }

    fn draw(&mut self, game: &mut Game) {}
}
