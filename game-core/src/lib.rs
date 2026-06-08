mod builder;
mod engine;
mod game;
mod input;
mod logistics;

pub use game::Game;

#[unsafe(no_mangle)]
pub fn init(ctx: &mut ggez::Context) -> Box<Game> {
    Box::new(Game::new(ctx))
}

#[unsafe(no_mangle)]
pub fn update(game: &mut Game, ctx: &mut ggez::Context) {
    game.update(ctx);
}

#[unsafe(no_mangle)]
pub fn render(game: &mut Game, ctx: &mut ggez::Context) {
    game.render(ctx);
}
