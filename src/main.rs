extern crate minifb;
extern crate nalgebra;

use minifb::{Key, Window, WindowOptions};
use nalgebra::Complex;
use std::f64::consts::PI;
use std::time::{Duration, Instant};
use crate::nalgebra::Normed;
const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITER: u32 = 1000;

fn julia(c: Complex<f64>, z: Complex<f64>) -> u32 {
    let mut z = z;
    for i in 0..MAX_ITER {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Julia Set - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_elapsed = start_time.elapsed().as_secs_f64();

        let c = Complex::new(
            0.7885 * (2.0 * PI * time_elapsed / 10.0).cos(),
            0.7885 * (2.0 * PI * time_elapsed / 10.0).sin(),
        );

        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = (i % WIDTH) as f64 / (WIDTH as f64) * 3.0 - 1.5;
            let y = (i / WIDTH) as f64 / (HEIGHT as f64) * 3.0 - 1.5;

            let z = Complex::new(x, y);
            let value = julia(c, z);

            let color = if value == MAX_ITER {
                0
            } else {
                let blue = (10 * (value % 256)) as u8;
                let green = (5 * (value % 256)) as u8;
                let red = (20 * (value % 256)) as u8;


                u32::from_le_bytes([0, blue, green, red])
            };

            *pixel = color;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }
