use crate::grid;
use crate::grid::{ActionResult, LevelData, PlayerAction, Type};

use ggez::glam::{Vec2, vec2};
use ggez::graphics::{self, Color};
use glam::{IVec2, ivec2};
use sparsey::Entity;

pub struct RenderOpts {
    color: Color,
    order: i32,
}

pub struct Game {
    world: sparsey::World,
    level: LevelState,
}

pub struct LevelState {
    player: sparsey::Entity,
    solved: bool,
}

impl Game {
    pub fn new() -> Self {
        let world = sparsey::World::builder()
            .register::<grid::Location>()
            .register::<grid::Type>()
            .register::<grid::Target>()
            .register::<grid::Box>()
            .register::<grid::Player>()
            .register::<RenderOpts>()
            .build();
        let mut s = Self {
            world,
            level: LevelState {
                player: sparsey::Entity::with_index(0),
                solved: false,
            },
        };
        s.load_test_level();
        s
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let kb = &ctx.keyboard;
        handle_grid_input_system(&mut self.world, kb, self.level.player);
        reconcile_level_state_system(&mut self.world, &mut self.level);
        Ok(())
    }

    pub fn render(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let grid_size = 50.;
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.world
            .for_each::<(&grid::Location, &RenderOpts)>(|(location, opts)| {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest(location.0.as_vec2() * grid_size)
                        .scale(vec2(grid_size, grid_size))
                        .z(opts.order)
                        .color(opts.color),
                );
            });
        let res = canvas.finish(ctx);
        res
    }

    fn load_test_level(&mut self) {
        // Hardcode the level for now so we can get to the interesting bits
        let level_data = LevelData {
            boxes: vec![ivec2(1, 2), ivec2(2, 2)],
            targets: vec![ivec2(3, 3), ivec2(4, 4)],
            walls: vec![ivec2(0, 1), ivec2(1, 0)],
            player: ivec2(5, 5),
        };

        for target_pos in level_data.targets {
            self.world.create((
                grid::Location(target_pos),
                grid::Target { covered: false },
                grid::Type::Target,
                RenderOpts {
                    color: Color::GREEN,
                    order: 0,
                },
            ));
        }

        for box_pos in level_data.boxes {
            self.world.create((
                grid::Location(box_pos),
                grid::Box {},
                grid::Type::Box,
                RenderOpts {
                    color: Color::BLUE,
                    order: 1,
                },
            ));
        }

        for wall_pos in level_data.walls {
            self.world.create((
                grid::Location(wall_pos),
                grid::Type::Wall,
                RenderOpts {
                    color: Color::WHITE,
                    order: 0,
                },
            ));
        }

        self.level.player = self.world.create((
            grid::Location(level_data.player),
            grid::Type::Player,
            grid::Player {},
            RenderOpts {
                color: Color::MAGENTA,
                order: 2,
            },
        ));
    }
}

pub fn handle_grid_input_system(
    world: &mut sparsey::World,
    kb: &ggez::input::keyboard::KeyboardContext,
    player_ent: Entity,
) {
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

    let result = handle_player_action(world, player_ent, action);
    match result {
        ActionResult::Failure { blocked_by } => {
            println!("action failed, blocked by entity: {:?}", blocked_by);
        }
        ActionResult::Success(updates) => {
            let mut locations = world.borrow_mut::<grid::Location>();
            for update in updates {
                if let Some(l) = locations.get_mut(update.entity) {
                    l.0 = update.new_pos;
                }
            }
        }
    }
}

fn handle_player_action(
    world: &mut sparsey::World,
    player_ent: Entity,
    action: PlayerAction,
) -> grid::ActionResult {
    match action {
        PlayerAction::MoveLeft => move_player(world, player_ent, ivec2(-1, 0)),
        PlayerAction::MoveRight => move_player(world, player_ent, ivec2(1, 0)),
        PlayerAction::MoveUp => move_player(world, player_ent, ivec2(0, -1)),
        PlayerAction::MoveDown => move_player(world, player_ent, ivec2(0, 1)),
    }
}

fn move_player(world: &mut sparsey::World, player_ent: Entity, delta: IVec2) -> grid::ActionResult {
    let player_pos = world
        .query_one::<&grid::Location>()
        .get(player_ent)
        .expect("could not find player location in world")
        .0;

    let adj_pos = player_pos + delta;
    let far_pos = adj_pos + delta;
    let mut maybe_adj_entity = None;
    let mut maybe_far_entity = None;
    world
        .query_all::<(Entity, &grid::Location)>()
        .exclude::<(&grid::Target, &grid::Player)>()
        .for_each(|(ent, loc)| {
            if loc.0 == adj_pos {
                maybe_adj_entity = Some(ent);
            } else if loc.0 == far_pos {
                maybe_far_entity = Some(ent);
            }
        });

    match (maybe_adj_entity, maybe_far_entity) {
        // Next cell is empty, just move the player
        (None, _) => ActionResult::Success(vec![grid::EntityUpdate {
            entity: player_ent,
            prev_pos: player_pos,
            new_pos: adj_pos,
        }]),
        // Next cell is occupied, if we can push the entity in that cell and the
        // cell beyond it is empty, move both the player and the pushed entity
        (Some(adj_ent), None) => {
            if world.query_one::<&grid::Box>().get(adj_ent).is_some() {
                ActionResult::Success(vec![
                    grid::EntityUpdate {
                        entity: player_ent,
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

// reconciles the level state after an action has been taken
// - check for win conditions
// - update target count
fn reconcile_level_state_system(world: &mut sparsey::World, level: &mut LevelState) {
    let mut total_targets = 0;
    let mut covered_targets = 0;

    // Reset target active states
    world.for_each::<&mut grid::Target>(|target| {
        total_targets += 1;
        target.covered = false;
    });

    world
        .query_all::<(&grid::Location, &mut RenderOpts)>()
        .include::<&grid::Box>()
        .for_each(|(box_loc, box_opts)| {
            let mut box_covers = false;
            for (target_loc, target) in world
                .query_all::<(&grid::Location, &mut grid::Target)>()
                .iter()
            {
                if box_loc.0 == target_loc.0 {
                    target.covered = true;
                    covered_targets += 1;
                    box_covers = true;
                    break;
                }
            }
            box_opts.color = if box_covers { Color::CYAN } else { Color::BLUE };
        });

    level.solved = covered_targets == total_targets;
}
