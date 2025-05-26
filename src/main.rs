mod canvas;
use canvas::Canvas;

mod util;
use util::ScreenCoord;

use console::Term;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let mut term = Term::stdout();
    term.hide_cursor().unwrap();
    let frametime = Duration::from_millis(17);

    let mut time: f32 = 0.0;

    loop {
        let size = term.size();
        let mut canvas = Canvas::new(size);

        let middle: ScreenCoord = <(u16, u16) as TryInto<ScreenCoord>>::try_into(size).unwrap() / 2;

        let end = middle + ((time.sin() * middle.y as f32) as i16, (time.cos() * middle.x as f32) as i16);
        canvas.draw_line(middle, end, b'#');

        //let x = middle.1 as i16 + (time.sin() * (middle.1 - 1) as f32) as i16;
        //let y = middle.0 as i16 + (time.cos() * (middle.0 - 1) as f32) as i16;
        //canvas.draw_line((middle.0 as i16, middle.1 as i16), (y, x), b'#');

        term.move_cursor_to(0, 0).unwrap();
        term.write(canvas.flat().as_slice()).unwrap();

        thread::sleep(frametime);
        time += frametime.as_secs_f32();
    }
}

