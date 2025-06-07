mod canvas;
use canvas::Canvas;

mod vector;
use vector::Float3;

mod transform;

mod threed;
use threed::{Camera, Model};

use std::thread;
use std::time::Duration;

fn main() {
    let frametime = Duration::from_millis(17);

    let mut cube = Model::load(include_str!("cube.obj"));

    let mut time: f32 = 0.0;

    loop {
        let mut canvas = Canvas::new();

        let camera = Camera::new(canvas.aspect_ratio(), 2.0);

        cube.set_rotation(Float3::new(time * 0.85, time, time * 0.65).into());
        cube.draw(&camera, &mut canvas);

        canvas.render();

        thread::sleep(frametime);
        time += frametime.as_secs_f32();
    }
}

