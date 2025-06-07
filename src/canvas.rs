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
        print!("{}{}", termion::cursor::Goto(1, 1), self.buffer.clone().into_iter().collect::<String>());
    }

    pub fn plot_raw(&mut self, coord: (u16, u16), character: char) {
        // TODO: Add some bounds checking
        let coord = self.coord_1d(coord);
        self.buffer[coord] = character;
    }

    pub fn plot(&mut self, coord: Float2, character: char) {
        // Map from -1 - 1 to 0 - 1
        let coord = (coord + Float2::new(1.0, 1.0)) / 2.0;

        // Map to screen
        let coord = coord * Float2::new((&self.size.0 - 1) as f32, (&self.size.1 - 1) as f32);

        // Round
        let coord = (coord.x as u16, coord.y as u16);

        self.plot_raw(coord, character);
    }

    // 2D coord to 1D buffer coord
    fn coord_1d(&self, coord: (u16, u16)) -> usize {
        // TODO: Add some bounds checking
        (coord.0 + coord.1 * self.size.0) as usize
    }
}

