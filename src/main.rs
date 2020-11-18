use crate::universe::{Universe};

use rand::Rng;
use rand::thread_rng;
use std::thread::sleep;
use std::time::Duration;
use crate::view::cls;

mod universe;
mod view;

fn main() {
    let h = 50;
    let w = 50;
    let mut rng = thread_rng();
    let mut uni = Universe::with_eval(h, w, |_, _| rng.gen());
    loop {
        cls();
        println!("{}", uni);
        uni = uni.next_gen();
        sleep(Duration::from_millis(400));
    }
}
