extern crate sensehat;

use sensehat::SenseHat;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut sense_hat = SenseHat::new().unwrap();
    let color = [[(255, 0, 0); 64], [(0, 255, 0); 64], [(0, 0, 255); 64]];
    for i in 0..3 {
        sense_hat.set_pixels(&color[i]);
        sleep(Duration::from_millis(1000));
    }
    sense_hat.clear(None);
}