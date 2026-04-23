use crate::ecs::{self, World};
use crate::grid;
use crate::grid::{ActionResult, LevelData, PlayerAction, Type};

use ggez::glam::{Vec2, vec2};
use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};

pub struct Game {
    world: ecs::World,
    player: ecs::Entity,
}

impl Game {
    pub fn new() -> Self {
        let mut world = ecs::World::new();
        world
            .register_component::<grid::Location>()
            .register_component::<grid::Type>()
            .register_component::<grid::Target>()
            .register_component::<grid::Box>();
        let mut s = Self {
            world,
            player: ecs::Entity::none(),
        };
        s.load_test_level();
        s
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let kb = &ctx.keyboard;
        handle_grid_input_system(&mut self.world, kb);
        update_box_statuses(&mut self.world);
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let grid_size = 50.;
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let locations = self.world.components::<grid::Location>();
        let boxes = self.world.components::<grid::Box>();
        for (ent, grid_type) in self.world.components::<grid::Type>().iter_ent() {
            let color = match grid_type {
                Type::Player => Color::MAGENTA,
                Type::Box => {
                    if boxes.get(ent).unwrap().covering_target {
                        Color::CYAN
                    } else {
                        Color::BLUE
                    }
                }
                Type::Wall => Color::WHITE,
                Type::Target => Color::GREEN,
            };

            let position = locations.get(ent).unwrap().0;
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(position.as_vec2() * grid_size)
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
            boxes: vec![ivec2(1, 1), ivec2(2, 2)],
            targets: vec![ivec2(3, 3)],
            walls: vec![ivec2(0, 1), ivec2(1, 0)],
            player: ivec2(5, 5),
        };

        for target_pos in level_data.targets {
            self.world
                .spawn_entity()
                .emplace(grid::Location(target_pos))
                .emplace(grid::Target {})
                .emplace(grid::Type::Target);
        }

        for box_pos in level_data.boxes {
            self.world
                .spawn_entity()
                .emplace(grid::Location(box_pos))
                .emplace(grid::Box {
                    covering_target: false,
                })
                .emplace(grid::Type::Box);
        }

        for wall_pos in level_data.walls {
            self.world
                .spawn_entity()
                .emplace(grid::Location(wall_pos))
                .emplace(grid::Type::Wall);
        }

        self.player = self
            .world
            .spawn_entity()
            .emplace(grid::Location(level_data.player))
            .emplace(grid::Type::Player)
            .entity();
    }
}

pub fn handle_grid_input_system(world: &mut World, kb: &ggez::input::keyboard::KeyboardContext) {
    let action = if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Left) {
        PlayerAction::MoveLeft
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Right) {
        PlayerAction::MoveRight
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Up) {
        PlayerAction::MoveUp
    } else if kb.is_key_just_pressed(ggez::input::keyboard::KeyCode::Down) {
        PlayerAction::MoveDown
    } else {
        return;
    };

    let result = handle_player_action(world, action);
    match result {
        ActionResult::Failure { blocked_by } => {
            println!("action failed, blocked by entity: {:?}", blocked_by);
        }
        ActionResult::Success(updates) => {
            let mut locations = world.components_mut::<grid::Location>();
            for update in updates {
                if let Some(l) = locations.get_mut(update.entity) {
                    l.0 = update.new_pos;
                }
            }
        }
    }
}

fn handle_player_action(world: &mut World, action: PlayerAction) -> grid::ActionResult {
    match action {
        PlayerAction::MoveLeft => move_player(world, ivec2(-1, 0)),
        PlayerAction::MoveRight => move_player(world, ivec2(1, 0)),
        PlayerAction::MoveUp => move_player(world, ivec2(0, -1)),
        PlayerAction::MoveDown => move_player(world, ivec2(0, 1)),
    }
}

fn move_player(world: &mut World, delta: IVec2) -> grid::ActionResult {
    let types = world.components::<grid::Type>();
    let locations = world.components::<grid::Location>();

    let player = types
        .iter_ent()
        .find(|(_, t)| **t == grid::Type::Player)
        .expect("could not find player entity in world")
        .0;

    let player_pos = locations.get(player).unwrap().0;
    let adj_pos = player_pos + delta;
    let far_pos = adj_pos + delta;
    let mut maybe_adj_entity = None;
    let mut maybe_far_entity = None;
    for (ent, loc) in locations.iter_ent() {
        if loc.0 == adj_pos && !types.get(ent).map_or(false, |t| *t == grid::Type::Target) {
            maybe_adj_entity = Some(ent);
        } else if loc.0 == far_pos && !types.get(ent).map_or(false, |t| *t == grid::Type::Target) {
            maybe_far_entity = Some(ent);
        }
    }

    match (maybe_adj_entity, maybe_far_entity) {
        // Next cell is empty, just move the player
        (None, _) => ActionResult::Success(vec![grid::EntityUpdate {
            entity: player,
            prev_pos: player_pos,
            new_pos: adj_pos,
        }]),
        // Next cell is occupied, if we can push the entity in that cell and the
        // cell beyond it is empty, move both the player and the pushed entity
        (Some(adj_ent), None) => {
            if world.components::<grid::Box>().exists(adj_ent) {
                ActionResult::Success(vec![
                    grid::EntityUpdate {
                        entity: player,
                        prev_pos: player_pos,
                        new_pos: adj_pos,
                    },
                    grid::EntityUpdate {
                        entity: adj_ent,
                        prev_pos: adj_pos,
                        new_pos: far_pos,
                    },
                ])
            } else {
                ActionResult::Failure {
                    blocked_by: adj_ent,
                }
            }
        }
        // Otherwise, there is nothing to do
        _ => ActionResult::Failure {
            blocked_by: maybe_adj_entity.unwrap_or(maybe_far_entity.unwrap()),
        },
    }
}

fn update_box_statuses(world: &mut World) {
    let locations = world.components::<grid::Location>();
    let target_positions = world
        .components::<grid::Target>()
        .iter_ent()
        .map(|ent| locations.get(ent.0).unwrap().0)
        .collect::<Vec<_>>();

    for (ent, box_comp) in world.components_mut::<grid::Box>().iter_ent_mut() {
        let box_pos = locations.get(ent).unwrap().0;
        let covering_target = target_positions.contains(&box_pos);
        box_comp.covering_target = covering_target;
    }
}
