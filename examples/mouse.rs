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
