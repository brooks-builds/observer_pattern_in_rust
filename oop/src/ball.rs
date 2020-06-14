use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Ball {
    mesh: Mesh,
    location: Point2<f32>,
}

impl Ball {
    pub fn new(context: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<Ball> {
        let radius = 15.0;

        Ok(Ball {
            mesh: Ball::create_mesh(context, radius)?,
            location: Point2::new(screen_width / 2.0, screen_height / 2.0),
        })
    }

    pub fn update(&self) {}
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
