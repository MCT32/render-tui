use crate::util::ScreenCoord;

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<Vec<u8>>,
}

impl Canvas {
    pub fn new(size: (u16, u16)) -> Self {
        Self {
            buffer: vec![vec![b' '; size.1 as usize]; size.0 as usize],
        }
    }

    pub fn inbounds(&self, coord: ScreenCoord) -> bool {
        coord.x >= 0 &&
        coord.y >= 0 &&
        coord.x < self.buffer[0].len() as i16 &&
        coord.y < self.buffer.len() as i16
    }

    pub fn plot(&mut self, coord: ScreenCoord, byte: u8) {
        if self.inbounds(coord) {
            self.buffer[coord.y as usize][coord.x as usize] = byte;
        }
    }

    pub fn flat(&self) -> Vec<u8> {
        self.buffer.concat()
    }

    pub fn draw_line(&mut self, p0: ScreenCoord, p1: ScreenCoord, stroke: u8) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            self.draw_line_h(p0, p1, stroke)
        } else {
            self.draw_line_v(p0, p1, stroke)
        }
    }

    fn draw_line_h(&mut self, p0: ScreenCoord, p1: ScreenCoord, stroke: u8) {
        let mut p0 = p0;
        let mut p1 = p1;

        if p0.x > p1.x {
            (p0.y, p1.y) = (p1.y, p0.y);
            (p0.x, p1.x) = (p1.x, p0.x);
        }

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;

        let dir = if dy < 0 {-1} else {1};

        let dy = dy * dir;

        if dx != 0 {
            let mut y = p0.y;
            let mut p = 2*dy - dx;
            for i in 0..=dx {
                self.plot((y, (p0.x + i)).into(), stroke);

                if p >= 0 {
                    y += dir;
                    p -= 2*dx;
                }

                p += 2*dy;
            }
        }
    }

    fn draw_line_v(&mut self, p0: ScreenCoord, p1: ScreenCoord, stroke: u8) {
        let mut p0 = p0;
        let mut p1 = p1;

        if p0.y > p1.y {
            (p0.y, p1.y) = (p1.y, p0.y);
            (p0.x, p1.x) = (p1.x, p0.x);
        }

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;

        let dir = if dx < 0 {-1} else {1};

        let dx = dx * dir;

        if dy != 0 {
            let mut x = p0.x;
            let mut p = 2*dx - dy;
            for i in 0..=dy {
                self.plot(((p0.y + i), x).into(), stroke);

                if p >= 0 {
                    x += dir;
                    p -= 2*dy;
                }

                p += 2*dx;
            }
        }
    }
}

