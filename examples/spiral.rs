extern crate sensehat;

use sensehat::*;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut sense_hat = SenseHat::new().unwrap();

    let r = (255, 0, 0);
    let g = (0, 255, 0);
    let b = (0, 0, 255);
    let w = (255, 255, 255);
    let l = [r, g, b, w];
    sense_hat.clear(None);
    sense_hat.low_light(true);

    loop {
        for x in l.iter() {
            for c in 0..4 {
                for i in 0..(7 - c * 2) {
                    // top
                    let _ = sense_hat.set_pixel(i + c, c, *x);
                    // bottom
                    let _ = sense_hat.set_pixel(7 - i - c, 7 - c, *x);
                    // left
                    let _ = sense_hat.set_pixel(c, 7 - i - c, *x);
                    // right
                    let _ = sense_hat.set_pixel(7 - c, i + c, *x);

                    sleep(Duration::from_millis(50));
                }
            }
        }
    }
}