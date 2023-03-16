use crate::keyboard_impl::*;
use enigo::{Enigo, KeyboardControllable};
use hbb_common::{
    anyhow,
    message_proto::{ControlKey, PhysKeyCode},
    ResultType,
};
use rdev::{EventType, Key};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref ENIGO: Arc<Mutex<Enigo>> = {
        Arc::new(Mutex::new(Enigo::new()))
    };
}

/// How to release key?
///
/// Only need to release all modifiers when
/// exiting or when the control is interrupted.
pub fn simulate_keyboard_event(keyboard_event: &KeyboardEvent) -> ResultType<()> {
    if let Some(raw_event) = keyboard_event.raw_event {
        if let Some(keycode) = keyboard_event.keycode.clone() {
            process_keycode(keycode, raw_event)
        } else {
            // In Windows
            // When press '1', keycode=Chr('1').
            // when release '1', keycode=None.
            Ok(())
        }
    } else {
        Err(anyhow::anyhow!("Failed to simulate {:?}", keyboard_event))
    }
}

pub fn release_modifiers() -> ResultType<()> {
    for phys in [
        PhysKeyCode::ShiftLeft,
        PhysKeyCode::ShiftRight,
        PhysKeyCode::ControlLeft,
        PhysKeyCode::ControlRight,
        PhysKeyCode::AltLeft,
        PhysKeyCode::AltRight,
        PhysKeyCode::MetaLeft,
        PhysKeyCode::MetaRight,
    ] {
        simulate_phys(phys, false)?;
    }

    Ok(())
}

fn process_keycode(keycode: KeyCode, raw_event: RawKeyboardEvent) -> ResultType<()> {
    match keycode {
        KeyCode::ControlKey(control_key) => {
            if control_key.is_modifier() {
                // Hold modifiers.
                simulate_phys(raw_event.phys, raw_event.press)
            } else {
                let cur_modifiers = Modifiers::get_current_modifiers();
                let raw_event_vec = cur_modifiers.diff_modifiers(&raw_event.modifiers);
                prepare_pressed_keys(&raw_event_vec)?;
                dbg!(&raw_event_vec);

                // Click control key.
                simulate_phys(raw_event.phys, true).ok();
                simulate_phys(raw_event.phys, false).ok();
                Ok(())
            }
        }
        KeyCode::Chr(chr) => {
            if !raw_event.press {
                return Ok(());
            }
            // let cur_modifiers = Modifiers::get_current_modifiers();
            // let raw_event_vec = cur_modifiers.diff_modifiers(&raw_event.modifiers);
            // prepare_pressed_keys(&raw_event_vec)?;

            let chr = char::from_u32(chr).ok_or(anyhow::anyhow!("Failed to get char: {}", chr))?;
            dbg!(chr);
            // simulate_char_without_modifiers(chr)
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Unsupported keycode: {:?}", keycode)),
    }
}

/// Maybe change the current modifiers.
fn simulate_char_without_modifiers(chr: char) -> ResultType<()> {
    let mut en = ENIGO.lock().unwrap();
    let key = enigo::Key::Layout(chr);
    en.key_down(key)?;
    en.key_up(key);
    Ok(())
}

fn simulate_phys(phys: PhysKeyCode, press: bool) -> ResultType<()> {
    let key = Key::from_phys(&phys)?;
    let event_type = if press {
        EventType::KeyPress(key)
    } else {
        EventType::KeyRelease(key)
    };
    rdev::simulate(&event_type).map_err(|_| anyhow::anyhow!("Failed to simulate {:?}", event_type))
}

/// restore_flag is used to restore the keyboard state.
fn prepare_pressed_keys(raw_event_vec: &Vec<RawKeyboardEvent>) -> ResultType<()> {
    for raw_event in raw_event_vec {
        simulate_phys(raw_event.phys, raw_event.press)?;
    }
    Ok(())
}

#[test]
fn test_simulate_phys_chr() -> ResultType<()> {
    simulate_phys(PhysKeyCode::ControlLeft, true)?;
    simulate_char_without_modifiers('a')?;
    simulate_phys(PhysKeyCode::ControlLeft, false)?;

    Ok(())
}

#[test]
fn test_simulate_alt_tab() -> ResultType<()> {
    simulate_keyboard_event(&KeyboardEvent {
        keycode: Some(KeyCode::ControlKey(ControlKey::Tab)),
        press: true,
        modifiers: Modifiers::NONE,
        raw_event: Some(RawKeyboardEvent {
            phys: PhysKeyCode::Tab,
            press: true,
            modifiers: Modifiers::ALT,
            sys_code: 0,
        }),
    })?;

    Ok(())
}

#[test]
fn test_simulate_alt_o() -> ResultType<()> {
    simulate_keyboard_event(&KeyboardEvent {
        keycode: Some(KeyCode::Chr('o' as u32)),
        press: true,
        modifiers: Modifiers::NONE,
        raw_event: Some(RawKeyboardEvent {
            // French keyboard.
            phys: PhysKeyCode::KeyO,
            press: true,
            modifiers: Modifiers::ALT,
            sys_code: 0,
        }),
    })?;
    release_modifiers()?;

    Ok(())
}

#[test]
fn test_prepare() -> ResultType<()> {
    let raw_event_vec = vec![RawKeyboardEvent {
        phys: PhysKeyCode::ControlLeft,
        press: true,
        modifiers: Modifiers::NONE,
        sys_code: 0,
    }];
    prepare_pressed_keys(&raw_event_vec)?;
    release_modifiers()?;

    Ok(())
}
