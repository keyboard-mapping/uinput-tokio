uinput
======
`/dev/uinput` high level wrapper.

Example
-------
The following example writes `hello world`.

```rust
extern crate uinput_tokio;
use uinput_tokio::event::keyboard;

use std::thread;
use std::time::Duration;

fn main() {
	let mut device = uinput_tokio::default().unwrap()
		.name("test").unwrap()
		.event(uinput_tokio::event::Keyboard::All).unwrap()
		.create().unwrap();

	thread::sleep(Duration::from_secs(1));

	device.click(&keyboard::Key::H).unwrap();
	device.click(&keyboard::Key::E).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::Space).unwrap();
	device.click(&keyboard::Key::W).unwrap();
	device.click(&keyboard::Key::O).unwrap();
	device.click(&keyboard::Key::R).unwrap();
	device.click(&keyboard::Key::L).unwrap();
	device.click(&keyboard::Key::D).unwrap();
	device.click(&keyboard::Key::Enter).unwrap();

	device.synchronize().unwrap();
}
```

Example mouse
-------------
```rust
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

```
