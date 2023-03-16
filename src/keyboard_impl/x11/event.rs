use enigo::{Enigo, KeyboardControllable};
use hbb_common::message_proto::PhysKeyCode;

use crate::{
    client::get_key_state,
    keyboard_impl::{KeyboardEvent, Modifiers, RawKeyboardEvent},
};

pub fn is_altgr_pressed() -> bool {
    todo!()
}

impl Modifiers {
    /// Modifiers in linux not left or right.
    pub fn get_current_modifiers() -> Modifiers {
        let mut modifiers = Modifiers::NONE;

        for (key, modifier) in [
            (enigo::Key::Shift, Modifiers::SHIFT),
            (enigo::Key::Control, Modifiers::CTRL),
            (enigo::Key::Alt, Modifiers::ALT),
            (enigo::Key::Meta, Modifiers::META),
        ] {
            if get_key_state(key) {
                modifiers |= modifier;
            }
        }

        modifiers
    }

    pub fn normalize_altgr(&self) -> Self {
        *self
    }

    /// Get the codes that should be clicked,
    /// modifiers of both side can be sync after clicking the keys.
    ///
    /// The modifers in the vec represent the active state of the remote modifier,
    /// compare it with the local modifiers.
    ///
    /// Linux: modifier hasn't left or right.
    pub fn diff_modifiers(&self, target_modifiers: &Modifiers) -> Vec<KeyboardEvent> {
        let target_modifiers = target_modifiers.trans_positional_mods();
        let cur_modifiers = self.trans_positional_mods();

        let mut raw_event_vec: Vec<KeyboardEvent> = vec![];

        for (modifier, phys) in [
            (Modifiers::CAPS, PhysKeyCode::CapsLock),
            (Modifiers::NUM, PhysKeyCode::NumLock),
        ] {
            let pressed = target_modifiers.contains(modifier);

            if pressed && !cur_modifiers.contains(modifier)
                || !pressed && cur_modifiers.contains(modifier)
            {
                raw_event_vec.push(KeyboardEvent::with_phys(phys, true));
                raw_event_vec.push(KeyboardEvent::with_phys(phys, false));
            }
            continue;
        }

        for (modifier, left_phys, right_phys) in [
            (
                Modifiers::SHIFT,
                PhysKeyCode::ShiftLeft,
                PhysKeyCode::ShiftRight,
            ),
            (
                Modifiers::CTRL,
                PhysKeyCode::ControlLeft,
                PhysKeyCode::ControlRight,
            ),
            (Modifiers::ALT, PhysKeyCode::AltLeft, PhysKeyCode::AltRight),
            (
                Modifiers::META,
                PhysKeyCode::MetaLeft,
                PhysKeyCode::MetaRight,
            ),
            (
                Modifiers::ALT_GR,
                PhysKeyCode::AltRight,
                PhysKeyCode::AltRight,
            ),
        ] {
            let pressed = target_modifiers.contains(modifier);
            if !pressed && cur_modifiers.contains(modifier) {
                raw_event_vec.push(KeyboardEvent::with_phys(left_phys, false));
                raw_event_vec.push(KeyboardEvent::with_phys(right_phys, false));
            }
            if pressed && !cur_modifiers.contains(modifier) {
                raw_event_vec.push(KeyboardEvent::with_phys(left_phys, true))
            }
        }

        raw_event_vec
    }
}

#[test]
fn test_modifiers() {
    let modifiers = Modifiers::get_current_modifiers();
    dbg!(modifiers);
}

#[test]
fn test_diff_modifiers() {
    let target_modifiers = Modifiers::LEFT_ALT;
    let modifiers = Modifiers::NONE;

    assert_eq!(
        vec![KeyboardEvent::with_phys(PhysKeyCode::AltLeft, true),],
        modifiers.diff_modifiers(&target_modifiers)
    );

    let target_modifiers = Modifiers::LEFT_ALT;
    let modifiers = Modifiers::LEFT_ALT;

    assert_eq!(
        Vec::<KeyboardEvent>::new(),
        modifiers.diff_modifiers(&target_modifiers)
    );

    let target_modifiers = Modifiers::NONE;
    let modifiers = Modifiers::LEFT_ALT;

    assert_eq!(
        vec![
            KeyboardEvent::with_phys(PhysKeyCode::AltLeft, false),
            KeyboardEvent::with_phys(PhysKeyCode::AltRight, false),
        ],
        modifiers.diff_modifiers(&target_modifiers)
    );
}
