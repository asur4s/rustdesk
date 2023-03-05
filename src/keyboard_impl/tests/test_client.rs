use crate::keyboard;
use anyhow;
use hbb_common::{
    anyhow,
    env_logger::*,
    log,
    message_proto::{KeyEvent, KeyboardMode},
};
use rdev::{Event, EventType, Key};
use std::{io::Write, net::TcpStream};

static TARGET_HOST: &'static str = "127.0.0.1";

fn send_key_event(key_event: &KeyEvent) -> anyhow::Result<()> {
    log::info!("key_event: {:?}", key_event);
    let mut stream = TcpStream::connect((TARGET_HOST, 7878))?;
    let raw_data: Vec<u8> = key_event.to_owned().try_into()?;

    stream.write_all(&raw_data)?;
    stream.flush()?;
    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

#[test]
fn test_keyboard_grab() -> anyhow::Result<()> {
    init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
    std::env::set_var("DISPLAY", ":0");

    let (sender, recv) = std::sync::mpsc::channel();
    if let Err(err) = rdev::start_grab_listen(move |event: Event| match event.event_type {
        EventType::KeyPress(key) | EventType::KeyRelease(key) => {
            if let Key::Unknown(keycode) = key {
                log::error!("rdev get unknown key, keycode is : {:?}", keycode);
            } else {
                sender.send(event).ok();
            }
            None
        }
        _ => Some(event),
    }) {
        log::error!("Failed to init rdev grab thread: {:?}", err);
    };
    
    rdev::enable_grab();
    loop {
        let event = recv.recv()?;
        let lock_modes = None;
        let keyboard_mode = KeyboardMode::Map;

        if keyboard::is_long_press(&event) {
            continue;
        }
        for key_event in keyboard::event_to_key_events(&event, keyboard_mode, lock_modes) {
            // todo:
            send_key_event(&key_event)?;
        }
    }
}
