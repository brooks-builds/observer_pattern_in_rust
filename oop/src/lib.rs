mod drawing_system;

use drawing_system::DrawingSystem;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

pub struct Game {
    drawing_system: DrawingSystem,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Game> {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        Ok(Game {
            drawing_system: DrawingSystem::new(screen_width, screen_height, context)?,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);
        // Draw code here...

        self.drawing_system.run(context)?;

        graphics::present(context)
    }
}
