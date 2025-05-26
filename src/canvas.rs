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

    pub fn plot(&mut self, coord: (u16, u16), byte: u8) {
        self.buffer[coord.0 as usize][coord.1 as usize] = byte;
    }

    pub fn flat(&self) -> Vec<u8> {
        self.buffer.concat()
    }

    pub fn draw_line(&mut self, p1: (i16, i16), p2: (i16, i16), stroke: u8) {
        if (p2.1 - p1.1).abs() > (p2.0 - p1.0).abs() {
            self.draw_line_h(p1, p2, stroke)
        } else {
            self.draw_line_v(p1, p2, stroke)
        }
    }

    fn draw_line_h(&mut self, p1: (i16, i16), p2: (i16, i16), stroke: u8) {
        let mut p1 = p1;
        let mut p2 = p2;

        if p1.1 > p2.1 {
            (p1.0, p2.0) = (p2.0, p1.0);
            (p1.1, p2.1) = (p2.1, p1.1);
        }

        let dx = p2.1 - p1.1;
        let dy = p2.0 - p1.0;

        let dir = if dy < 0 {-1} else {1};

        let dy = dy * dir;

        if dx != 0 {
            let mut y = p1.0;
            let mut p = 2*dy - dx;
            for i in 0..=dx {
                self.plot((y.try_into().unwrap(), (p1.1 + i).try_into().unwrap()), stroke);

                if p >= 0 {
                    y += dir;
                    p -= 2*dx;
                }

                p += 2*dy;
            }
        }
    }

    fn draw_line_v(&mut self, p1: (i16, i16), p2: (i16, i16), stroke: u8) {
        let mut p1 = p1;
        let mut p2 = p2;

        if p1.0 > p2.0 {
            (p1.0, p2.0) = (p2.0, p1.0);
            (p1.1, p2.1) = (p2.1, p1.1);
        }

        let dx = p2.1 - p1.1;
        let dy = p2.0 - p1.0;

        let dir = if dx < 0 {-1} else {1};

        let dx = dx * dir;

        if dy != 0 {
            let mut x = p1.1;
            let mut p = 2*dx - dy;
            for i in 0..=dy {
                self.plot(((p1.0 + i).try_into().unwrap(), x.try_into().unwrap()), stroke);

                if p >= 0 {
                    x += dir;
                    p -= 2*dy;
                }

                p += 2*dx;
            }
        }
    }
}

