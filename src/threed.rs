use crate::canvas::Canvas;

use crate::vector::{Float2, Float3};
use crate::transform::{Transform, Rotation};

pub struct Model {
    verts: Vec<Float3>,
    vert_normals: Vec<Float3>,
    faces: Vec<(usize, usize, usize)>,

    transform: Transform
}

impl Model {
    pub fn load(file: &str) -> Self {
        let mut model = Self {
            verts: vec![],
            vert_normals: vec![],
            faces: vec![],
            transform: Default::default(),
        };

        for line in file.lines() {
            let parts: Vec<_> = line.split(' ').collect();

            match parts[0] {
                "v" => {
                    model.verts.push(
                        Float3::new(parts[1].parse::<f32>().unwrap(), parts[2].parse::<f32>().unwrap(), parts[3].parse::<f32>().unwrap())
                    )
                },
                "vn" => {
                    model.vert_normals.push(
                        Float3::new(parts[1].parse::<f32>().unwrap(), parts[2].parse::<f32>().unwrap(), parts[3].parse::<f32>().unwrap())
                    )
                },
                "f" => {
                    model.faces.push(
                        (
                            parts[1].split('/').collect::<Vec<_>>()[0].parse::<usize>().unwrap(),
                            parts[2].split('/').collect::<Vec<_>>()[0].parse::<usize>().unwrap(),
                            parts[3].split('/').collect::<Vec<_>>()[0].parse::<usize>().unwrap(),
                        )
                    )
                },
                _ => (),
            };
        };

        model
    }

    pub fn draw(&self, camera: &Camera, canvas: &mut Canvas) {
        let screen_verts: Vec<Float2> = self.verts.iter().map(|vert| {
            // Apply transform
            let vert = self.transform.apply(*vert);

            camera.world_to_screen(vert)
        }).collect();

        let mut index: u8 = 0;
        for face in &self.faces {
            canvas.draw_tri(screen_verts[face.0 - 1], screen_verts[face.1 - 1], screen_verts[face.2 - 1]);
            if index < 254 { index += 1 } else { index = 0 };
        }

        //for vert in &screen_verts {
        //    canvas.plot(*vert, 'O');
        //}
    }

    pub fn set_transform(&mut self, transform: Transform) {self.transform = transform;}

    pub fn set_rotation(&mut self, rotation: Rotation) {self.transform.rotation = rotation;}
}

pub struct Camera {
    aspect_ratio: f32,
    fov: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, fov: f32) -> Self {
        Self{ aspect_ratio, fov }
    }

    pub fn world_to_screen(&self, vert: Float3) -> Float2 {
        Float2::from((vert.x, vert.z)) / Float2::new(self.aspect_ratio, 1.0) / self.fov
    }
}

