use std::{time::Instant, sync::mpsc};
use rppal::gpio::{Gpio, Trigger};

const PIN: u8 = 4;

fn main() {
    let gpio = Gpio::new().unwrap();
	let mut pin = gpio.get(PIN).unwrap().into_input();

	let start = Instant::now();

	let (tx, rx) = mpsc::channel();

	pin.set_async_interrupt(
		Trigger::Both,
		move |level| {
			tx.send((start.elapsed().as_secs_f64(), level)).unwrap();
		}
	).unwrap();

	loop {
		let (time, level) = rx.recv().unwrap();
		println!("{}: {}", time, level);
	}
}
