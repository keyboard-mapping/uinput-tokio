extern crate uinput_tokio;
use uinput_tokio::event::keyboard;

use std::thread;
use std::time::Duration;

use tokio;

#[tokio::main]
async fn main() {
    let mut device = uinput_tokio::default()
        .await
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
