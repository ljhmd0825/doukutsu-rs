use crate::game::Game;

use super::backend::{init_backend, BackendRenderer};
use super::error::GameResult;
use super::filesystem::Filesystem;
use super::gamepad::GamepadContext;
use super::graphics::SwapMode;
use super::keyboard::KeyboardContext;

pub struct Context {
    pub headless: bool,
    pub shutdown_requested: bool,
    pub size_hint: (u16, u16),
    pub window_title: String,
    pub preferred_renderer: Option<String>,
    pub(crate) filesystem: Filesystem,
    pub(crate) renderer: Option<Box<dyn BackendRenderer>>,
    pub(crate) gamepad_context: GamepadContext,
    pub(crate) keyboard_context: KeyboardContext,
    pub(crate) real_screen_size: (u32, u32),
    pub(crate) screen_size: (f32, f32),
    pub(crate) screen_insets: (f32, f32, f32, f32),
    pub(crate) swap_mode: SwapMode,
}

impl Context {
    pub fn new() -> Context {
        Context {
            headless: false,
            shutdown_requested: false,
            size_hint: (640, 480),
            window_title: "Game".to_string(),
            preferred_renderer: None,
            filesystem: Filesystem::new(),
            renderer: None,
            gamepad_context: GamepadContext::new(),
            keyboard_context: KeyboardContext::new(),
            real_screen_size: (320, 240),
            screen_size: (320.0, 240.0),
            screen_insets: (0.0, 0.0, 0.0, 0.0),
            swap_mode: SwapMode::VSync,
        }
    }

    pub fn run(&mut self, game: &mut Game) -> GameResult {
        let backend = init_backend(self.headless, self.size_hint)?;
        let mut event_loop = backend.create_event_loop(self)?;
        self.renderer = Some(event_loop.new_renderer()?);

        event_loop.run(game, self);

        Ok(())
    }
}
