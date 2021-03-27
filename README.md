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

use tokio;

#[tokio::main]
async fn main() {
    let mut device = uinput_tokio::default()
        .unwrap()
        .name("test")
        .unwrap()
        .event(uinput_tokio::event::Keyboard::All)
        .unwrap()
        .create()
        .await
        .unwrap();

    thread::sleep(Duration::from_secs(1));

    device.click(&keyboard::Key::H).await.unwrap();
    device.click(&keyboard::Key::E).await.unwrap();
    device.click(&keyboard::Key::L).await.unwrap();
    device.click(&keyboard::Key::L).await.unwrap();
    device.click(&keyboard::Key::O).await.unwrap();
    device.click(&keyboard::Key::Space).await.unwrap();
    device.click(&keyboard::Key::W).await.unwrap();
    device.click(&keyboard::Key::O).await.unwrap();
    device.click(&keyboard::Key::R).await.unwrap();
    device.click(&keyboard::Key::L).await.unwrap();
    device.click(&keyboard::Key::D).await.unwrap();
    device.click(&keyboard::Key::Enter).await.unwrap();

    device.synchronize().await.unwrap();
}

```

Example mouse
-------------
```rust
extern crate uinput_tokio;

use std::thread;
use std::time::Duration;
use tokio;
use uinput_tokio::event::controller::Controller::Mouse;
use uinput_tokio::event::controller::Mouse::Left;
use uinput_tokio::event::relative::Position::{X, Y};
use uinput_tokio::event::relative::Relative::Position;
use uinput_tokio::event::Event::{Controller, Relative};

#[tokio::main]
async fn main() {
    let mut device = uinput_tokio::default()
        .unwrap()
        .name("test")
        .unwrap()
        .event(Controller(Mouse(Left)))
        .unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Relative(Position(X)))
        .unwrap()
        .event(Relative(Position(Y)))
        .unwrap()
        .create()
        .await
        .unwrap();

    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));

        device.send(X, 50).await.unwrap();
        device.send(Y, 50).await.unwrap();
        device.synchronize().await.unwrap();
    }
}

```
