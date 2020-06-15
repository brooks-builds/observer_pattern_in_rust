use super::{EventSystem, GameEvent};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Ball {
    mesh: Mesh,
    location: Point2<f32>,
    velocity: Point2<f32>,
    radius: f32,
}

impl Ball {
    pub fn new(context: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Ball> {
        let radius = 15.0;

        Ok(Ball {
            mesh: Ball::create_mesh(context, radius)?,
            location: Point2::new(screen_width / 2.0, screen_height / 2.0),
            velocity: Point2::new(0.1, 0.1),
            radius,
        })
    }

    pub fn update(&mut self, screen_width: f32, screen_height: f32, event_system: &EventSystem) {
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;

        if self.location.y + self.radius >= screen_height {
            self.velocity.y *= -1.0;
        } else if self.location.y - self.radius <= 0.0 {
            self.velocity.y *= -1.0;
        }

        if self.location.x + self.radius >= screen_width {
            self.velocity.x *= -1.0;
            event_system.notify(GameEvent::PlayerScored);
        } else if self.location.x - self.radius <= 0.0 {
            self.velocity.x *= -1.0;
            event_system.notify(GameEvent::AiScored);
        }
    }
    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::default().dest(self.location),
        )
    }

    fn create_mesh(context: &mut Context, radius: f32) -> GameResult<Mesh> {
        MeshBuilder::new()
            .circle(
                DrawMode::fill(),
                Point2::new(0.0, 0.0),
                radius,
                0.01,
                Color::new(1.0, 1.0, 1.0, 1.0),
            )
            .build(context)
    }
}
