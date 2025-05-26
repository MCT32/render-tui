mod canvas;
use canvas::Canvas;

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
        canvas.draw_line((0, 0), (
            (size.0-1).try_into().unwrap(),
            (size.1-1).try_into().unwrap()),
        b'#');

        term.move_cursor_to(0, 0).unwrap();
        term.write(canvas.flat().as_slice()).unwrap();

        thread::sleep(frametime);
        time += frametime.as_secs_f32();
    }
}

