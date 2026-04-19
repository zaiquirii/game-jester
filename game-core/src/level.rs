use crate::ecs::Entity;
use glam::{IVec2, ivec2};

pub struct LevelData {
    pub dimensions: IVec2,
    pub boxes: Vec<IVec2>,
    pub walls: Vec<IVec2>,
    pub targets: Vec<IVec2>,
    pub player: IVec2,
}

#[derive(Clone, Copy, Debug)]
pub enum GridType {
    Player,
    Box,
    Wall,
}

#[derive(Clone, Copy)]
pub struct GridEntity {
    pub entity: Entity,
    pub grid_type: GridType,
    pub position: IVec2,
}

pub struct GridEntityInfo {
    entity: GridEntity,
    index: usize,
}

pub struct GridTarget {
    entity:
}

pub struct SokoGrid {
    width: usize,
    height: usize,
    player: GridEntity,
    entities: Vec<GridEntity>,
    targets: Vec<GridEntity>,
}

impl SokoGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            player: GridEntity {
                entity: Entity::none(),
                position: IVec2::ZERO,
                grid_type: GridType::Player,
            },
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity, grid_type: GridType, position: IVec2) {
        self.entities.push(GridEntity {
            entity,
            position,
            grid_type: grid_type,
        });
    }

    pub fn set_player(&mut self, entity: Entity, position: IVec2) {
        self.player = GridEntity {
            entity,
            position,
            grid_type: GridType::Player,
        };
    }

    pub fn accept_action(&mut self, action: PlayerAction) -> ActionResult {
        match action {
            PlayerAction::MoveLeft => self.move_player(-1, 0),
            PlayerAction::MoveRight => self.move_player(1, 0),
            PlayerAction::MoveUp => self.move_player(0, -1),
            PlayerAction::MoveDown => self.move_player(0, 1),
        }
    }

    fn ent_at(&self, pos: IVec2) -> Option<GridEntityInfo> {
        for (idx, entity) in self.entities.iter().enumerate() {
            if entity.position == pos {
                return Some(GridEntityInfo {
                    entity: *entity,
                    index: idx,
                });
            }
        }
        if self.player.position == pos {
            return Some(GridEntityInfo {
                entity: self.player,
                // This will cause an out of bounds error if used.
                index: usize::MAX,
            });
        }
        None
    }

    fn move_player(&mut self, dx: i32, dy: i32) -> ActionResult {
        let mut updates = Vec::new();

        let delta = ivec2(dx, dy);
        let target_player_pos = self.player.position + delta;

        let entity = self.ent_at(target_player_pos);
        match entity {
            None => {
                self.player.position = target_player_pos;
                updates.push(EntityUpdate {
                    entity: self.player,
                    previous_position: self.player.position,
                });
            }
            Some(pushed_entity) => match pushed_entity.entity.grid_type {
                GridType::Player => unreachable!(),
                GridType::Wall => {
                    return ActionResult::Failure {
                        blocked_by: pushed_entity.entity,
                    };
                }
                GridType::Box => {
                    let target_backing_pos = target_player_pos + delta;
                    match self.ent_at(target_backing_pos) {
                        None => {
                            self.player.position = target_player_pos;
                            updates.push(EntityUpdate {
                                entity: self.player,
                                previous_position: self.player.position,
                            });
                            self.entities[pushed_entity.index].position = target_backing_pos;
                            updates.push(EntityUpdate {
                                entity: self.entities[pushed_entity.index],
                                previous_position: target_player_pos,
                            })
                        }
                        Some(backing_ent) => {
                            return ActionResult::Failure {
                                blocked_by: backing_ent.entity,
                            };
                        }
                    }
                }
            },
        }
        ActionResult::Success(updates)
    }
}

pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    // Undo,
}

pub enum ActionResult {
    Success(Vec<EntityUpdate>),
    Failure { blocked_by: GridEntity },
}

pub struct EntityUpdate {
    pub entity: GridEntity,
    pub previous_position: IVec2,
}
