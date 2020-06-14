use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct DividingLine {
    mesh: Mesh,
    location: Point2<f32>,
}

impl DividingLine {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        context: &mut Context,
    ) -> GameResult<DividingLine> {
        let width = 5.0;
        let height = screen_height;
        let location = Point2::new(screen_width / 2.0 - width / 2.0, 0.0);
        Ok(DividingLine {
            mesh: DividingLine::create_mesh(width, height, context)?,
            location,
        })
    }

    pub fn create_mesh(width: f32, height: f32, context: &mut Context) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(0.0, 0.0, width, height),
                Color::new(1.0, 1.0, 1.0, 0.2),
            )
            .build(context)
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<()> {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::default().dest(self.location),
        )
    }
}
