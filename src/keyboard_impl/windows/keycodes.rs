use crate::keyboard_impl::KeyOps;
use hbb_common::{
    anyhow::{self, anyhow},
    ResultType,
};
use rdev::Key;

impl KeyOps for Key {
    fn pos_code(&self) -> ResultType<u32> {
        rdev::win_scancode_from_key(*self)
            .ok_or_else(|| anyhow!("Failed to get scancode for {:?}", self))
    }

    fn sys_code(&self) -> ResultType<u32> {
        rdev::win_keycode_from_key(*self)
            .ok_or_else(|| anyhow!("Failed to get vk code for {:?}", self))
    }

    fn from_pos(code: u32) -> ResultType<Key> {
        match rdev::win_key_from_scancode(code) {
            Key::Unknown(code) => anyhow::bail!("Unknown scancode: {}", code),
            key => Ok(key),
        }
    }

    fn from_event(event: &rdev::Event) -> ResultType<Key> {
        let code = event.scan_code;
        Self::from_pos(code)
    }
}

#[test]
fn test_from_code() -> ResultType<()> {
    // ShiftLeft: scancode = 42(0x2A) in Windows
    assert_eq!(Key::ShiftLeft, Key::from_pos(42)?);
    Ok(())
}
