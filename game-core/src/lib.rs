pub struct State {
    counter: u32,
}

impl State {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }
}

#[unsafe(no_mangle)]
pub fn init() -> Box<State> {
    Box::new(State::new())
}

#[unsafe(no_mangle)]
pub fn update(state: &mut State) {
    state.increment()
}

#[unsafe(no_mangle)]
pub fn render(state: &mut State) {
    println!("Counter: {}", state.get_counter());
}
