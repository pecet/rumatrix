use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use rumatrix::*;
use rumatrix::config::Config;
use rumatrix::renderer::{CrossTermRenderer, Renderer};
use rumatrix::state::State;

fn main() {
    let config = Rc::new(Config {
        size_x: Some(80),
        size_y: Some(20),
        symbols: "ABCDEFGHIJKL1234567890".chars().collect()
    });
    let mut state = Rc::new(State::new(&config));
    let renderer: Box<dyn Renderer> = Box::new(
        CrossTermRenderer::new(&state, &config)
    );
    renderer.init();
    // TODO: add exit condition
    let mut previous_start = Instant::now();
    let mut start = previous_start;
    loop {
        let start = Instant::now();
        renderer.render();
        thread::sleep(Duration::from_millis(5));
        let took = Instant::now().duration_since(previous_start);
        state.advance(took.as_secs_f64());
        previous_start = start;
    }
    renderer.clean_up();
}
