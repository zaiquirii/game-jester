use std::fmt::format;

use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};
use glam::{Vec2, vec2};
use sparsey::Entity;

use crate::{builder, engine};
use crate::{input, logistics};

pub struct Game {
    world: sparsey::World,
    resources: engine::Resources,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let mut world = sparsey::World::builder()
            .register::<engine::Location>()
            .register::<engine::SpriteShape>()
            .register::<engine::AnimatedSprite>()
            .register::<logistics::NetworkNode>()
            .build();

        let mut sprites = engine::Sprites::new(ctx);

        sprites
            .load_sprite_sheet(ctx, "test_sprite", "/sprites/test_sprite")
            .unwrap();

        sprites
            .load_sprite_sheet(ctx, "knight", "/sprites/knight")
            .unwrap();

        world.create((
            engine::Location(vec2(300., 250.)),
            engine::SpriteShape {
                shape: engine::Shape::Circle { radius: 25.0 },
                color: Color::GREEN,
                filled: true,
            },
            logistics::NetworkNode {
                network: 1,
                range: 100.,
            },
        ));

        let mut resources = engine::Resources::new();
        resources
            .register(sprites)
            .register(engine::DebugLines::new(ctx).unwrap())
            .register(input::State::default())
            .register(builder::State::new(&mut world))
            .register(logistics::Networks::new());

        Self { world, resources }
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        const DESIRED_FPS: u32 = 60;
        input::update_state_system(ctx, &self.resources);
        builder::update_state_system(&mut self.world, &self.resources);

        while ctx.time.check_update_time(DESIRED_FPS) {
            let delta_ms = 1000.0 / DESIRED_FPS as f32;
            engine::advance_sprite_animations_system(
                &mut self.world,
                &self.resources.get::<engine::Sprites>(),
                delta_ms,
            );
            logistics::update_network_system(
                &mut self.world,
                &mut self.resources.get_mut::<engine::DebugLines>(),
            );
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        engine::render_sprites_system(
            &mut canvas,
            &mut self.world,
            &self.resources.get::<engine::Sprites>(),
        );

        let offset = (ctx.time.ticks() % 100) * 2;
        let mut debug_lines = self.resources.get_mut::<engine::DebugLines>();
        debug_lines.add_circle(vec2(25., 25.), 25., Color::RED);
        debug_lines.add_line(
            vec2(25.0, 25.0),
            vec2(300.0, 60.0 + offset as f32),
            Color::GREEN,
        );
        debug_lines.draw(&mut canvas);
        let res = canvas.finish(ctx);
        debug_lines.clear();
        res
    }
}
