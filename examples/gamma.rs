extern crate sensehat;

use sensehat::*;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut sense_hat = SenseHat::new().unwrap();

    sense_hat.clear(Some((255, 127, 0)));
    let mut gamma = sense_hat.gamma();
    println!("{:?}", gamma);
    sleep(Duration::from_secs(2));

    gamma[..].reverse();
    sense_hat.set_gamma(&gamma).unwrap();
    println!("{:?}", gamma);
    sleep(Duration::from_secs(2));

    sense_hat.low_light(true);
    println!("{:?}", gamma);
    sleep(Duration::from_secs(2));

    sense_hat.low_light(false);
}