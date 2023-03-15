use winapi::um::processthreadsapi::GetCurrentThreadId;
use winapi::um::winuser::{
    AttachThreadInput, GetForegroundWindow, GetKeyboardState, GetWindowThreadProcessId,
    VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN,
};
use std::ptr::null_mut;
use crate::keyboard::IS_0X021D_DOWN;
use crate::keyboard_impl::{Modifiers, FALSE, TRUE};

pub fn is_altgr_pressed() -> bool {
    unsafe { IS_0X021D_DOWN }
}

impl Modifiers{
    pub fn get_current_modifiers() -> Modifiers {
        /// get_key_state will get the incorrect modifier state when press AltGr.
        let mut modifiers = Modifiers::NONE;

        let window_thread_id =
            unsafe { GetWindowThreadProcessId(GetForegroundWindow(), null_mut()) };
        let thread_id = unsafe { GetCurrentThreadId() };

        let mut states = [0u8; 256];

        unsafe {
            if AttachThreadInput(thread_id, window_thread_id, TRUE) == 1 {
                // Current state of the modifiers in keyboard
                GetKeyboardState(states.as_mut_ptr());
                AttachThreadInput(thread_id, window_thread_id, FALSE);
            }
        }
        for (vk_code, modifier) in [
            // todo: check vk_shift, vk_ctrl, vk_alt
            // (VK_SHIFT, Modifiers::SHIFT),
            (VK_LSHIFT, Modifiers::LEFT_SHIFT),
            (VK_RSHIFT, Modifiers::RIGHT_SHIFT),
            // (VK_CONTROL, Modifiers::CTRL),
            (VK_LCONTROL, Modifiers::LEFT_CTRL),
            (VK_RCONTROL, Modifiers::RIGHT_CTRL),
            // (VK_MENU, Modifiers::ALT),
            (VK_LMENU, Modifiers::LEFT_ALT),
            (VK_RMENU, Modifiers::RIGHT_ALT),
            (VK_LWIN, Modifiers::META),
            (VK_RWIN, Modifiers::META),
        ] {
            if Self::is_pressed(&states, vk_code) {
                modifiers |= modifier;
            }
        }

        modifiers
    }

    #[inline]
    fn is_pressed(states: &[u8; 256], vk_code: i32) -> bool {
        (states[vk_code as usize] & 0x80) != 0
    }

    pub fn normalize_altgr(&self) -> Self {
        let mut modifiers = self.clone();
        if modifiers.contains(Modifiers::LEFT_CTRL) && modifiers.contains(Modifiers::RIGHT_ALT) {
            modifiers = modifiers - Modifiers::LEFT_CTRL - Modifiers::RIGHT_ALT;
            modifiers |= Modifiers::ALT_GR;
        }
        modifiers
    }
}