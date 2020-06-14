mod dividing_line;

use dividing_line::DividingLine;
use ggez::{Context, GameResult};

pub struct DrawingSystem {
    dividing_line: DividingLine,
}

impl DrawingSystem {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        context: &mut Context,
    ) -> GameResult<DrawingSystem> {
        Ok(DrawingSystem {
            dividing_line: DividingLine::new(screen_width, screen_height, context)?,
        })
    }

    pub fn run(&self, context: &mut Context) -> GameResult<()> {
        self.dividing_line.draw(context)
    }
}
