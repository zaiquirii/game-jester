use std::{fmt::UpperExp, mem::transmute, path::Path, process::Command};

use game_core::Game;
use ggez::{ContextBuilder, conf, event, input::keyboard::KeyCode};
use libloading::{Library, Symbol};

const LIB_PATH: &str = "./target/debug/libgame_core.dylib";

// MAIN DEV
fn main() -> anyhow::Result<()> {
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("jester_hot_reload", "zaiquiri")
        .default_conf(c)
        .build()?;

    let lib = DynamicGameFuncs::load(LIB_PATH)?;
    let state = unsafe { (lib.init)() };
    let event_handler = HotReloadEventHandler {
        game_state: *state,
        lib,
    };

    event::run(ctx, event_loop, event_handler);
    Ok(())
}

fn cargo(args: &[&str]) -> anyhow::Result<()> {
    println!("Building project...");
    let exit_status = Command::new("cargo").args(args).spawn()?.wait()?;
    if exit_status.success() {
        Ok(())
    } else {
        anyhow::bail!("Failed to compile project")
    }
}

struct HotReloadEventHandler {
    game_state: Game,
    lib: DynamicGameFuncs,
}

impl ggez::event::EventHandler for HotReloadEventHandler {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if ctx.keyboard.is_key_just_pressed(KeyCode::R) {
            match cargo(&["build", "-p", "game-core"]) {
                Ok(_) => {
                    println!("Build successful! Reloading library...");
                    self.lib = DynamicGameFuncs::load(LIB_PATH).unwrap();
                }
                Err(e) => println!("Error building project: {}", e),
            }
        }
        unsafe {
            (self.lib.update)(&mut self.game_state);
        }
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        unsafe {
            (self.lib.render)(&mut self.game_state);
        }
        Ok(())
    }
}

// DynamicGameFuncs is a wrapper around the dynamically loaded library and its function pointers
// specifically to allow hot reloading without needing to worry about the library's lifetime or
// the validity of the function pointers. By keeping the library as a field, we ensure that it
// remains loaded in memory as long as we need to call its functions, and we can easily reload it
// when needed. The function pointers are transmuted to have a 'static lifetime, which simplifies
// their usage throughout the code without needing to worry about lifetimes or borrowing issues.
struct DynamicGameFuncs {
    // We need to keep the library around to ensure the symbols remain valid
    _lib: Library,
    init: Symbol<'static, unsafe extern "C" fn() -> Box<Game>>,
    update: Symbol<'static, unsafe extern "C" fn(&mut Game) -> ggez::GameResult>,
    render: Symbol<'static, unsafe extern "C" fn(&mut Game) -> ggez::GameResult>,
}

impl DynamicGameFuncs {
    fn load(path: &str) -> anyhow::Result<Self> {
        unsafe {
            let lib = Library::new(path)?;
            let init: Symbol<unsafe extern "C" fn() -> Box<Game>> = lib.get(b"init")?;
            let init = std::mem::transmute(init);
            let update: Symbol<unsafe extern "C" fn(&mut Game) -> ggez::GameResult> =
                lib.get(b"update")?;
            let update = std::mem::transmute(update);
            let render: Symbol<unsafe extern "C" fn(&mut Game) -> ggez::GameResult> =
                lib.get(b"render")?;
            let render = std::mem::transmute(render);
            Ok(Self {
                _lib: lib,
                init,
                update,
                render,
            })
        }
    }
}
