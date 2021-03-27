extern crate uinput_tokio;

use std::thread;
use std::time::Duration;
use uinput_tokio::event::controller::Controller::Mouse;
use uinput_tokio::event::controller::Mouse::Left;
use uinput_tokio::event::Event::{Controller, Relative};
use uinput_tokio::event::relative::Position::{X, Y};
use uinput_tokio::event::relative::Relative::Position;

fn main() {
	let mut device = uinput_tokio::default().unwrap()
		.name("test").unwrap()
		.event(Controller(Mouse(Left))).unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
		.event(Relative(Position(X))).unwrap()
		.event(Relative(Position(Y))).unwrap()
		.create().unwrap();

	for _ in 1..10 {
		thread::sleep(Duration::from_secs(1));

		device.send(X, 50).unwrap();
		device.send(Y, 50).unwrap();
		device.synchronize().unwrap();
	}
}
