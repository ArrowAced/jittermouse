use enigo::{Enigo, MouseControllable};
use rand::{distributions::Uniform, distributions::Distribution};
use std::{thread, time::Duration};

fn main() {
    let time: Uniform<i32> = Uniform::from(9000..10000);
    let mut mouse = Enigo::new();
    let mut dir: usize = 0;
    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    loop {
        sleep_random(&time);
        mouse.mouse_move_relative(directions[dir % 4].0, directions[dir % 4].1);
        dir = dir.wrapping_add(1);
    }
}

fn sleep_random(range: &Uniform<i32>) {
    let mut rng = rand::thread_rng();
    thread::sleep(Duration::from_millis(range.sample(&mut rng) as u64));
}
