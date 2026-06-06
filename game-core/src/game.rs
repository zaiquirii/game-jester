use std::fmt::format;

use ggez::glam::{Vec2, vec2};
use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};
use sparsey::Entity;

use crate::debug;
use crate::engine;

pub struct Game {
    world: sparsey::World,
    sprites: engine::Sprites,
    debug_lines: debug::DebugLines,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let mut world = sparsey::World::builder()
            .register::<engine::Location>()
            .register::<engine::AnimatedSprite>()
            .build();

        let mut sprite_manager = engine::Sprites::new();

        sprite_manager
            .load_sprite_sheet(ctx, "test_sprite", "/sprites/test_sprite")
            .unwrap();

        sprite_manager
            .load_sprite_sheet(ctx, "knight", "/sprites/knight")
            .unwrap();

        let debug_lines = debug::DebugLines::new(ctx).unwrap();
        let mut s = Self {
            world,
            sprites: sprite_manager,
            debug_lines,
        };
        s
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            let delta_ms = 1000.0 / DESIRED_FPS as f32;
            engine::advance_sprite_animations_system(&mut self.world, &mut self.sprites, delta_ms)
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        engine::render_animated_sprite_system(&mut canvas, &mut self.world, &mut self.sprites);
        let offset = (ctx.time.ticks() % 100) * 2;
        self.debug_lines.add_circle(vec2(25., 25.), 25., Color::RED);
        self.debug_lines.add_line(
            vec2(25.0, 25.0),
            vec2(300.0, 60.0 + offset as f32),
            Color::GREEN,
        );
        self.debug_lines.draw(&mut canvas);
        let res = canvas.finish(ctx);
        self.debug_lines.clear();
        res
    }
}
