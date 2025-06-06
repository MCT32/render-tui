mod canvas;
use canvas::Canvas;

mod threed;
use threed::{Camera, Model};

use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let frametime = Duration::from_millis(17);

    let mut cube = Model::cube((-1.0, -1.0, -1.0).into(), (1.0, 1.0, 1.0).into());

    let mut time: f32 = 0.0;

    loop {
        let mut canvas = Canvas::new();

        let camera = Camera::new(canvas.aspect_ratio(), 2.0);

        cube.set_rotation((time, 0.0, 0.0).into());
        cube.draw(&camera, &mut canvas);

        canvas.render();

        thread::sleep(frametime);
        time += frametime.as_secs_f32();
    }
}

