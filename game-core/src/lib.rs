mod ecs;
mod game;
mod level;

pub use game::Game;

#[unsafe(no_mangle)]
pub fn init() -> Box<Game> {
    Box::new(Game::new())
}

#[unsafe(no_mangle)]
pub fn update(game: &mut Game, ctx: &mut ggez::Context) -> ggez::GameResult {
    game.update(ctx)
}

#[unsafe(no_mangle)]
pub fn render(game: &mut Game, ctx: &mut ggez::Context) -> ggez::GameResult {
    game.render(ctx)
}
