use ggez::glam::{Vec2, vec2};
use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};
use sparsey::Entity;

pub struct Game {
    world: sparsey::World,
}

impl Game {
    pub fn new() -> Self {
        let world = sparsey::World::builder().build();
        let mut s = Self { world };
        s
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let grid_size = 50.;
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let res = canvas.finish(ctx);
        res
    }
}
