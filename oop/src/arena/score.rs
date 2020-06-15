use super::{GameEvent, Observer};
use ggez::graphics::{DrawParam, Font, Scale, Text};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use std::sync::{Arc, Mutex};

pub type WrappedScore = Arc<Mutex<Score>>;

pub struct Score {
    player_text: Text,
    ai_text: Text,
    player_location: Point2<f32>,
    ai_location: Point2<f32>,
    player: u8,
    ai: u8,
}

impl Score {
    pub fn new(screen_width: f32, context: &mut Context) -> GameResult<WrappedScore> {
        let player = 0;
        let ai = 0;
        let player_text = Score::create_text(player);
        let (player_text_width, _player_text_height) = player_text.dimensions(context);
        let ai_text = Score::create_text(ai);
        Ok(Arc::new(Mutex::new(Score {
            player_text,
            ai_text,
            player_location: Point2::new(screen_width / 2.0 - player_text_width as f32 - 5.0, 0.0),
            ai_location: Point2::new(screen_width / 2.0 + 3.0, 0.0),
            player,
            ai,
        })))
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.player_text,
            DrawParam::default().dest(self.player_location),
        )?;
        graphics::draw(
            context,
            &self.ai_text,
            DrawParam::default().dest(self.ai_location),
        )
    }

    fn create_text(score: u8) -> Text {
        let mut score_text = Text::new(format!("{}", score));
        score_text.set_font(Font::default(), Scale::uniform(50.0));
        score_text
    }
}

impl Observer for Score {
    fn on_notify(&mut self, event: &GameEvent) {
        match event {
            GameEvent::PlayerScored => {
                self.player += 1;
                self.player_text = Self::create_text(self.player);
            }
            GameEvent::AiScored => {
                self.ai += 1;
                self.ai_text = Self::create_text(self.ai);
            }
        }
    }
}
