mod dividing_line;
mod score;

use super::{EventSystem, GameEvent, Observer};
use dividing_line::DividingLine;
use ggez::{Context, GameResult};
use score::{Score, WrappedScore};

pub struct Arena {
    dividing_line: DividingLine,
    wrapped_score: WrappedScore,
}

impl Arena {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        context: &mut Context,
        event_system: &mut EventSystem,
    ) -> GameResult<Arena> {
        let wrapped_score = Score::new(screen_width, context)?;

        event_system.add_observer(wrapped_score.clone());

        Ok(Arena {
            dividing_line: DividingLine::new(screen_width, screen_height, context)?,
            wrapped_score,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        self.dividing_line.draw(context)?;

        {
            let score = self.wrapped_score.lock().unwrap();
            score.draw(context)
        }
    }
}
