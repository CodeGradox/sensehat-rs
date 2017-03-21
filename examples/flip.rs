extern crate sensehat;

use sensehat::SenseHat;
use sensehat::display::{Orientation, Pixel};

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let w = (255, 255, 255);
    let b = (0, 128, 255);
    let smile: [Pixel; 64] = [
        w, w, w, w, w, b, b, b,
        b, w, b, b, w, b, b, b,
        b, b, w, b, w, b, b, b,
        b, b, b, w, w, w, w, w,
        b, w, w, w, w, b, b, w,
        b, b, b, w, b, w, b, w,
        b, b, b, w, b, b, w, w,
        b, b, b, b, b, b, b, w
    ];

    let mut sense_hat = SenseHat::new().unwrap();
    sense_hat.set_pixels(&smile);

    sleep(Duration::from_millis(2000));
    sense_hat.flip_h(true);
    sleep(Duration::from_millis(2000));
    sense_hat.flip_v(true);
    sleep(Duration::from_millis(2000));
    sense_hat.clear(None);
}