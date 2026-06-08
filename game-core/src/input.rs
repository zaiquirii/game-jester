use ggez::event::MouseButton;

use crate::engine;

pub struct State {
    selector_pos: glam::Vec2,
    select_pressed: bool,
}

impl State {
    // selector_pos is the position in the world that the "selector" is pointing to. While this is
    // often the mouse, putting this indirection allows to the easy change of the selector position
    // by something else (gamepad/stick)
    pub fn selector_pos(&self) -> glam::Vec2 {
        self.selector_pos
    }

    pub fn select_pressed(&self) -> bool {
        self.select_pressed
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            selector_pos: glam::Vec2::ZERO,
            select_pressed: false,
        }
    }
}

pub fn update_state_system(ctx: &mut ggez::Context, resources: &engine::Resources) {
    let mut state = resources.get_mut::<State>();

    let mouse_screen_pos = ctx.mouse.position();
    state.selector_pos = mouse_screen_pos.into();
    state.select_pressed = ctx.mouse.button_just_pressed(MouseButton::Left);
}
