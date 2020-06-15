mod arena;
mod ball;
mod event_system;

use arena::Arena;
use ball::Ball;
use event_system::{EventSystem, GameEvent, Observer};
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

pub struct Game {
    arena: Arena,
    ball: Ball,
    event_system: EventSystem,
}

impl Game {
    pub fn new(context: &mut Context) -> GameResult<Game> {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let mut event_system = EventSystem::new();
        Ok(Game {
            arena: Arena::new(screen_width, screen_height, context, &mut event_system)?,
            ball: Ball::new(context, screen_width, screen_height)?,
            event_system,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (screen_width, screen_height) = graphics::drawable_size(context);

        self.ball
            .update(screen_width, screen_height, &self.event_system);
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
