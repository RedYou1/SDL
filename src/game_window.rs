use std::time::Duration;

use sdl2::{render::Canvas, video::Window};

use crate::event::Event;

pub trait GameWindow {
    fn running(&mut self) -> bool;
    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        width: f32,
        height: f32,
    ) -> Result<(), String>;
    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String>;
    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String>;
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}
