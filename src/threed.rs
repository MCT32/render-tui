use crate::canvas::Canvas;

use crate::vector::{Float2, Float3};

pub struct Model {
    verts: Vec<Float3>,
    lines: Vec<(usize, usize)>,

    rotation: Float3,
}

impl Model {
    pub fn draw(&self, camera: &Camera, canvas: &mut Canvas) {
        for vert in &self.verts {
            let coord = camera.world_to_screen(vert);
            canvas.plot(coord, 'o');
        }
    }

    pub fn set_rotation(&mut self, rotation: Float3) {self.rotation = rotation;}

    pub fn cube(start: Float3, end: Float3) -> Self {
        Self {
            verts: vec![
                Float3::new(start.x, start.y, start.z),
                Float3::new(end.x,   start.y, start.x),
                Float3::new(end.x,   end.y,   start.x),
                Float3::new(start.x, end.y,   start.x),
                Float3::new(start.x, start.y, end.x  ),
                Float3::new(end.x,   start.y, end.x  ),
                Float3::new(end.x,   end.y,   end.x  ),
                Float3::new(start.x, end.y,   end.x  ),
            ],
            lines: vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
            ],
            rotation: Float3::zero()
        }
    }
}

pub struct Camera {
    aspect_ratio: f32,
    fov: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, fov: f32) -> Self {
        Self{ aspect_ratio, fov }
    }

    pub fn world_to_screen(&self, vert: &Float3) -> Float2 {
        Float2::from((vert.x, vert.z)) / Float2::new(self.aspect_ratio, 1.0) / self.fov
    }
}

