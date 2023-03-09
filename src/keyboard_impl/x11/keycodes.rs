use rdev::Key;
use crate::keyboard_impl::KeyOps;

impl KeyOps for Key {
    fn from_pos(code: u32) -> Option<Key> {
        let key = rdev::linux_key_from_code(code);
        match key {
            Key::Unknown(_) => None,
            _ => Some(key),
        }
    }

    fn pos_code(&self) {
        todo!()
    }

    fn sys_code(&self) {
        todo!()
    }

    fn from_event(event: &rdev::Event) -> Option<Key> {
        let code = event.code as u32;
        Self::from_pos(code)
    }
}

#[test]
fn test_from_code() {
    dbg!(Key::from_pos(50));
}
