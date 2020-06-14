mod arena;
mod ball;

use arena::Arena;
use ball::Ball;
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

pub struct Game {
    arena: Arena,
    ball: Ball,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Game> {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        Ok(Game {
            arena: Arena::new(screen_width, screen_height, context)?,
            ball: Ball::new(context, screen_width, screen_height)?,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.ball.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);
        // Draw code here...

        self.arena.draw(context)?;
        self.ball.draw(context)?;

        graphics::present(context)
    }
}
