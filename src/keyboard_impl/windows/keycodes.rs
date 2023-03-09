use crate::keyboard_impl::KeyOps;
use rdev::Key;

impl KeyOps for Key {
    fn from_pos(code: u32) -> Option<Key> {
        let key = rdev::win_key_from_scancode(code);
        match key {
            Key::Unknown(_) => None,
            _ => Some(key),
        }
    }

    fn from_event(event: &rdev::Event) -> Option<Key> {
        let code = event.scan_code;
        Self::from_pos(code)
    }

    fn pos_code(&self) {
        todo!()
    }

    fn sys_code(&self) {
        todo!()
    }
}

#[test]
fn test_from_code() {
    // ShiftLeft: scancode = 42(0x2A)
    assert_eq!(Some(Key::ShiftLeft), Key::from_pos(42))
}