use std::cmp::{min, max};

use crate::vector::Float2;

pub struct Canvas {
    size: (u16, u16),
    aspect_ratio: f32,
    buffer: Vec<char>,
}

impl Canvas {
    pub fn new() -> Self {
        // TODO: Propegate these errors
        let size = termion::terminal_size().unwrap();
        let size_pixels = termion::terminal_size_pixels().unwrap();

        let aspect_ratio = size_pixels.0 as f32 / size_pixels.1 as f32;
        let buffer = vec![' '; (size.0 * size.1) as usize];

        Self {
            size,
            aspect_ratio,
            buffer,
        }
    }

    pub fn size(&self) -> (u16, u16) { self.size }

    pub fn aspect_ratio(&self) -> f32 { self.aspect_ratio }

    pub fn render(&self) {
        print!("{}{}", termion::cursor::Goto(1, 1), self.buffer.iter().fold(String::new(), |a, b| format!("{}{}", a, b.to_string())));
    }

    pub fn plot_raw(&mut self, coord: (u16, u16), pixel: char) {
        if coord.0 < self.size.0 && coord.1 < self.size.1 {
            let coord = self.coord_1d(coord);
            self.buffer[coord] = pixel;
        }
    }

    pub fn plot(&mut self, coord: Float2, pixel: char) {
        self.plot_raw(self.coord_to_pixel(coord), pixel);
    }

    pub fn coord_to_pixel(&self, coord: Float2) -> (u16, u16) {
        // Map from -1 - 1 to 0 - 1
        let coord = (coord + Float2::new(1.0, 1.0)) / 2.0;

        // Map to screen
        let coord = coord * Float2::new((&self.size.0 - 1) as f32, (&self.size.1 - 1) as f32);

        // Round
        (coord.x as u16, coord.y as u16)
    }

    fn sign(p0: Float2, p1: Float2, p2: Float2) -> f32 {
        (p0.x - p2.x) * (p1.y - p2.y)
        - (p1.x - p2.x) * (p0.y - p2.y)
    }

    fn is_in_tri(point: (u16, u16), p0: (u16, u16), p1: (u16, u16), p2: (u16, u16)) -> bool {
        let point = Float2::new(point.0 as f32, point.1 as f32);
        let p0 = Float2::new(p0.0 as f32, p0.1 as f32);
        let p1 = Float2::new(p1.0 as f32, p1.1 as f32);
        let p2 = Float2::new(p2.0 as f32, p2.1 as f32);

        let d0 = Self::sign(point, p0, p1);
        let d1 = Self::sign(point, p1, p2);
        let d2 = Self::sign(point, p2, p0);

        let has_neg = (d0 < 0.0) || (d1 < 0.0) || (d2 < 0.0);
        let has_pos = (d0 > 0.0) || (d1 > 0.0) || (d2 > 0.0);

        !(has_neg && has_pos)
    }

    pub fn draw_tri_raw(&mut self,
            p0: (u16, u16), p1: (u16, u16), p2: (u16, u16),
        ) {
        let bounds = (
            (min(min(p0.0, p1.0), p2.0), min(min(p0.1, p1.1), p2.1)),
            (max(max(p0.0, p1.0), p2.0), max(max(p0.1, p1.1), p2.1))
        );

        let p0f = Float2::new(p0.0 as f32, p0.1 as f32);
        let p1f = Float2::new(p1.0 as f32, p1.1 as f32);
        let p2f = Float2::new(p2.0 as f32, p2.1 as f32);

        if Self::sign(p1f, p2f, p0f) >= 0.0 {return};

        for x in bounds.0.0..=bounds.1.0 {
            for y in bounds.0.1..=bounds.1.1 {
                if Self::is_in_tri((x, y), p0, p1, p2) {
                    // Shading
                    let gradient = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

                    let brightness = (x - bounds.0.0) as f32 / (bounds.1.0 - bounds.0.0) as f32;
                    let level = (brightness * (gradient.len() - 1) as f32).floor() as usize;
                    let fill = gradient.chars().nth(level).unwrap().into();

                    self.plot_raw((x, y), fill)
                };
            }
        }
    }

    pub fn draw_tri(&mut self, p0: Float2, p1: Float2, p2: Float2) {
        self.draw_tri_raw(self.coord_to_pixel(p0), self.coord_to_pixel(p1), self.coord_to_pixel(p2));
    }

    // 2D coord to 1D buffer coord
    fn coord_1d(&self, coord: (u16, u16)) -> usize {
        // TODO: Add some bounds checking
        (coord.0 + coord.1 * self.size.0) as usize
    }
}

