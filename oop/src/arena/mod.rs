mod dividing_line;
mod score;

use dividing_line::DividingLine;
use ggez::{Context, GameResult};
use score::Score;

pub struct Arena {
    dividing_line: DividingLine,
    score: Score,
}

impl Arena {
    pub fn new(screen_width: f32, screen_height: f32, context: &mut Context) -> GameResult<Arena> {
        Ok(Arena {
            dividing_line: DividingLine::new(screen_width, screen_height, context)?,
            score: Score::new(screen_width, context)?,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        self.dividing_line.draw(context)?;
        self.score.draw(context)
    }
}
