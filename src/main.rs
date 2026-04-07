use std::process::Command;

use game_core::State;
use libloading::{Library, Symbol};

// MAIN DEV
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut lib = unsafe { Library::new("./target/debug/libgame_core.dylib")? };
    let mut game_funcs = HotLoadedGameFuncs::load(&lib)?;

    let mut state = unsafe { (game_funcs.init)() };
    let mut c = 0;
    loop {
        c += 1;
        if c % 1_000_000 == 0 {
            match cargo(&["build", "-p", "game-core"]) {
                Ok(_) => {
                    // SAFETY: We just built the library, so it should be safe to load and call its functions.
                    unsafe {
                        lib = Library::new("./target/debug/libgame_core.dylib")?;
                        game_funcs = HotLoadedGameFuncs::load(&lib)?;
                    }
                }
                Err(e) => println!("Error building project: {}", e),
            }
        }
        unsafe {
            (game_funcs.render)(state.as_mut());
            (game_funcs.update)(state.as_mut());
        }
    }
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

struct HotLoadedGameFuncs<'lib> {
    init: Symbol<'lib, unsafe extern "C" fn() -> Box<State>>,
    update: Symbol<'lib, unsafe extern "C" fn(&mut State)>,
    render: Symbol<'lib, unsafe extern "C" fn(&mut State)>,
}

impl<'a> HotLoadedGameFuncs<'a> {
    fn load(lib: &'a Library) -> anyhow::Result<Self> {
        unsafe {
            Ok(Self {
                init: lib.get(b"init")?,
                update: lib.get(b"update")?,
                render: lib.get(b"render")?,
            })
        }
    }
}
