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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = uinput_tokio::default()?
        .name("test")?
        .event(uinput_tokio::event::Keyboard::All)?
        .create()
        .await?;

    thread::sleep(Duration::from_secs(1));

    device.click(&keyboard::Key::H).await?;
    device.click(&keyboard::Key::E).await?;
    device.click(&keyboard::Key::L).await?;
    device.click(&keyboard::Key::L).await?;
    device.click(&keyboard::Key::O).await?;
    device.click(&keyboard::Key::Space).await?;
    device.click(&keyboard::Key::W).await?;
    device.click(&keyboard::Key::O).await?;
    device.click(&keyboard::Key::R).await?;
    device.click(&keyboard::Key::L).await?;
    device.click(&keyboard::Key::D).await?;
    device.click(&keyboard::Key::Enter).await?;

    device.synchronize().await
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = uinput_tokio::default()?
        .name("test")?
        .event(Controller(Mouse(Left)))?
        // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Relative(Position(X)))?
        .event(Relative(Position(Y)))?
        .create()
        .await?;

    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));

        device.send(X, 50).await?;
        device.send(Y, 50).await?;
        device.synchronize().await?;
    }
    Ok(())
}
```
