use crate::ecs::{self, World};
use crate::level::{GridEntityType, LevelData, PlayerAction, SokoGrid};

use ggez::glam::{Vec2, vec2};
use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};

pub struct Game {
    world: ecs::World,
    current_level: Option<SokoGrid>,
}

impl Game {
    pub fn new() -> Self {
        let mut world = ecs::World::new();
        world.register_component::<GridRenderable>();
        let mut s = Self {
            world,
            current_level: None,
        };
        s.load_test_level();
        s
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let kb = &ctx.keyboard;
        if let Some(level) = &mut self.current_level {
            handle_grid_input_system(&mut self.world, level, kb);
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let grid_size = 50.;
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        for renderable in self.world.iter::<GridRenderable>() {
            let color = match renderable.grid_type {
                GridEntityType::Player => Color::GREEN,
                GridEntityType::Box => Color::RED,
            };
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(Vec2::new(
                        renderable.position.x as f32 * grid_size,
                        renderable.position.y as f32 * grid_size,
                    ))
                    .scale(vec2(grid_size, grid_size))
                    .color(color),
            );
        }
        let res = canvas.finish(ctx);
        res
    }

    fn load_test_level(&mut self) {
        // Hardcode the level for now so we can get to the interesting bits
        let level_data = LevelData {
            dimensions: ivec2(5, 5),
            boxes: vec![ivec2(1, 1), ivec2(2, 2)],
            targets: vec![ivec2(3, 3)],
            player: ivec2(5, 5),
        };

        let mut level = SokoGrid::new(
            level_data.dimensions.x as usize,
            level_data.dimensions.y as usize,
        );

        for box_pos in level_data.boxes {
            let box_ent = self.world.spawn_entity();
            level.add_box(box_ent, box_pos);
            self.world.emplace(
                box_ent,
                GridRenderable {
                    grid_type: GridEntityType::Box,
                    position: box_pos,
                },
            );
        }

        let player_ent = self.world.spawn_entity();
        level.set_player(player_ent, level_data.player);
        self.world.emplace(
            player_ent,
            GridRenderable {
                grid_type: GridEntityType::Player,
                position: level_data.player,
            },
        );
        self.current_level = Some(level);
    }
}

struct GridRenderable {
    grid_type: GridEntityType,
    position: IVec2,
}

pub fn handle_grid_input_system(
    world: &mut World,
    current_level: &mut SokoGrid,
    kb: &ggez::input::keyboard::KeyboardContext,
) {
    let action = if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Left) {
        Some(PlayerAction::MoveLeft)
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Right) {
        Some(PlayerAction::MoveRight)
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Up) {
        Some(PlayerAction::MoveUp)
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Down) {
        Some(PlayerAction::MoveDown)
    } else {
        None
    };

    if let Some(action) = action {
        if let Some(updates) = current_level.accept_action(action) {
            for update in updates {
                println!(
                    "entity: {:?}, type: {:?}, position: {:?}",
                    update.entity.entity, update.entity.entity_type, update.entity.position
                );
                world
                    .get_mut::<GridRenderable>(update.entity.entity)
                    .expect(
                        "attempted to update entity which does not have a GridRenderable component",
                    )
                    .position = update.entity.position;
            }
        }
    }
}
