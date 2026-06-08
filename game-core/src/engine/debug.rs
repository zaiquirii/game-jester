use ggez::graphics;
use glam::{Vec2, vec2};

pub struct DebugLines {
    circle_mesh: graphics::Mesh,
    circles: Vec<(Vec2, f32, graphics::Color)>,
    lines: graphics::InstanceArray,
}

impl DebugLines {
    pub fn new(ctx: &mut ggez::Context) -> Result<Self, ggez::GameError> {
        let circle_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            glam::Vec2::ZERO,
            0.5,
            0.005,
            graphics::Color::WHITE,
        )?;

        let lines = graphics::InstanceArray::new(ctx, None);

        Ok(Self {
            circle_mesh,
            lines,
            circles: Vec::new(),
        })
    }

    pub fn add_line(&mut self, start: Vec2, end: Vec2, color: graphics::Color) -> &mut Self {
        let length = (end - start).length();
        let angle = vec2(1., 0.).angle_to(end - start);
        self.lines.push(
            graphics::DrawParam::new()
                .dest(start)
                .rotation(angle)
                .scale(vec2(length, 5.))
                .color(color),
        );
        self
    }

    pub fn add_circle(&mut self, center: Vec2, radius: f32, color: graphics::Color) -> &mut Self {
        self.circles.push((center, radius, color));
        self
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.circles.clear();
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(&self.lines, graphics::DrawParam::default());
        for (center, radius, color) in &self.circles {
            canvas.draw(
                &self.circle_mesh,
                graphics::DrawParam::new()
                    .color(*color)
                    .dest(*center)
                    .scale(vec2(*radius, *radius)),
            );
        }
    }
}
