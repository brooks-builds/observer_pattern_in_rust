use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

pub struct Game {
    // Your state here...
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
		    // ...
		}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw code here...
        graphics::present(ctx)
    }
}
