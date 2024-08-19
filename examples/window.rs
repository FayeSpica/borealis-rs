use std::error::Error;

use winit::event_loop::EventLoop;

fn main() -> Result<(), Box<dyn Error>> {
    borealis_rs::main(EventLoop::new().unwrap())
}
