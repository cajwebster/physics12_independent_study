use rppal::gpio::{Gpio, Level, Trigger};
use std::{sync::mpsc, time::Instant};

const PIN: u8 = 4;

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(PIN).unwrap().into_input();

    let start = Instant::now();

    let mut pulses: Vec<(Level, f64)> = vec![];

    let (tx, rx) = mpsc::channel();

    pin.set_async_interrupt(Trigger::Both, move |level| {
        tx.send((start.elapsed().as_secs_f64(), level)).unwrap();
    })
    .unwrap();

    for i in 1..=50 {
        let (time, level) = rx.recv().unwrap();
        println!("{:3}: {:.4}: {}", i, time, level);
        pulses.push((level, time));
    }

    let mut t1 = vec![];
    let mut t2 = vec![];

    for ((level1, time1), (level2, time2)) in pulses.windows(2).map(|slice| (slice[0], slice[1])) {
        match (level1, level2) {
            (Level::High, Level::Low) => t1.push(time2 - time1),
            (Level::Low, Level::High) => t2.push(time2 - time1),
            _ => (),
        }
    }

    let t1_avg = t1.iter().sum::<f64>() / t1.len() as f64;
    let t2_avg = t2.iter().sum::<f64>() / t2.len() as f64;

    println!(
        "t1: {:.4e} s, t2: {:.4e} s, f: {:.4e} hz",
        t1_avg,
        t2_avg,
        1.0 / (t1_avg + t2_avg)
    );
}
