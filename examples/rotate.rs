extern crate sensehat;

use sensehat::*;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut sense_hat = SenseHat::new().unwrap();

    sense_hat.clear(None);
    sense_hat.set_rotation(Orientation::Deg270, false);

    let color = (255, 0, 0);
    for i in 0..8 {
        sense_hat.set_pixel(i, 0, color).unwrap();
    }

    sleep(Duration::from_millis(1000));
    sense_hat.set_rotation(Orientation::Deg0, true);
    sleep(Duration::from_millis(1000));
    sense_hat.set_rotation(Orientation::Deg180, true);
    sleep(Duration::from_millis(1000));
    sense_hat.set_rotation(Orientation::Deg90, true);
    sleep(Duration::from_millis(1000));
    sense_hat.clear(None);
}