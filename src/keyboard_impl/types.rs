use bitflags::*;
use hbb_common::message_proto::ControlKey;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

bitflags! {
    /// https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    /// Use xmodmap -pm to get meaning of modifier
    #[derive(Default, Deserialize, Serialize)]
    pub struct Modifiers: u16 {
        const NONE = 0;

        const SHIFT = 1<<1;
        const ALT = 1<<2;
        const CTRL = 1<<3;
        const META = 1<<4;

        const LEFT_ALT = 1<<5;
        const RIGHT_ALT = 1<<6;
        const LEFT_CTRL = 1<<7;
        const RIGHT_CTRL = 1<<8;
        const LEFT_SHIFT = 1<<9;
        const RIGHT_SHIFT = 1<<10;

        const CAPS = 1<<11;
        const NUM = 1<<12;

        const ALT_GR = 1<<13;
    }
}

impl TryFrom<String> for Modifiers {
    type Error = String;

    fn try_from(s: String) -> Result<Modifiers, String> {
        let mut mods = Modifiers::NONE;

        let modifier_vec: Vec<_> = s.split('|').map(|ele| ele.trim()).collect();
        for (value, label) in [
            (Self::NONE, "NONE"),
            (Self::SHIFT, "SHIFT"),
            (Self::ALT, "ALT"),
            (Self::CTRL, "CTRL"),
            (Self::META, "META"),
            (Self::LEFT_ALT, "LEFT_ALT"),
            (Self::RIGHT_ALT, "RIGHT_ALT"),
            (Self::LEFT_CTRL, "LEFT_CTRL"),
            (Self::RIGHT_CTRL, "RIGHT_CTRL"),
            (Self::LEFT_SHIFT, "LEFT_SHIFT"),
            (Self::RIGHT_SHIFT, "RIGHT_SHIFT"),
            (Self::CAPS, "CAPS"),
            (Self::NUM, "NUM"),
            (Self::ALT_GR, "ALT_GR"),
        ] {
            if modifier_vec.contains(&label) {
                mods |= value;
            }
        }

        Ok(mods)
    }
}

/// Keycode for key events.
#[derive(Clone, Debug)]
pub enum KeyCode {
    ControlKey(ControlKey),
    Chr(u32),
    _Raw(u32),
}

lazy_static::lazy_static! {
    pub static ref KEY_MAP: HashMap<&'static str, KeyCode> =
    [
        ("VK_A", KeyCode::Chr('a' as _)),
        ("VK_B", KeyCode::Chr('b' as _)),
        ("VK_C", KeyCode::Chr('c' as _)),
        ("VK_D", KeyCode::Chr('d' as _)),
        ("VK_E", KeyCode::Chr('e' as _)),
        ("VK_F", KeyCode::Chr('f' as _)),
        ("VK_G", KeyCode::Chr('g' as _)),
        ("VK_H", KeyCode::Chr('h' as _)),
        ("VK_I", KeyCode::Chr('i' as _)),
        ("VK_J", KeyCode::Chr('j' as _)),
        ("VK_K", KeyCode::Chr('k' as _)),
        ("VK_L", KeyCode::Chr('l' as _)),
        ("VK_M", KeyCode::Chr('m' as _)),
        ("VK_N", KeyCode::Chr('n' as _)),
        ("VK_O", KeyCode::Chr('o' as _)),
        ("VK_P", KeyCode::Chr('p' as _)),
        ("VK_Q", KeyCode::Chr('q' as _)),
        ("VK_R", KeyCode::Chr('r' as _)),
        ("VK_S", KeyCode::Chr('s' as _)),
        ("VK_T", KeyCode::Chr('t' as _)),
        ("VK_U", KeyCode::Chr('u' as _)),
        ("VK_V", KeyCode::Chr('v' as _)),
        ("VK_W", KeyCode::Chr('w' as _)),
        ("VK_X", KeyCode::Chr('x' as _)),
        ("VK_Y", KeyCode::Chr('y' as _)),
        ("VK_Z", KeyCode::Chr('z' as _)),
        ("VK_0", KeyCode::Chr('0' as _)),
        ("VK_1", KeyCode::Chr('1' as _)),
        ("VK_2", KeyCode::Chr('2' as _)),
        ("VK_3", KeyCode::Chr('3' as _)),
        ("VK_4", KeyCode::Chr('4' as _)),
        ("VK_5", KeyCode::Chr('5' as _)),
        ("VK_6", KeyCode::Chr('6' as _)),
        ("VK_7", KeyCode::Chr('7' as _)),
        ("VK_8", KeyCode::Chr('8' as _)),
        ("VK_9", KeyCode::Chr('9' as _)),
        ("VK_COMMA", KeyCode::Chr(',' as _)),
        ("VK_SLASH", KeyCode::Chr('/' as _)),
        ("VK_SEMICOLON", KeyCode::Chr(';' as _)),
        ("VK_QUOTE", KeyCode::Chr('\'' as _)),
        ("VK_LBRACKET", KeyCode::Chr('[' as _)),
        ("VK_RBRACKET", KeyCode::Chr(']' as _)),
        ("VK_BACKSLASH", KeyCode::Chr('\\' as _)),
        ("VK_MINUS", KeyCode::Chr('-' as _)),
        ("VK_PLUS", KeyCode::Chr('=' as _)), // it is =, but sciter return VK_PLUS
        ("VK_DIVIDE", KeyCode::ControlKey(ControlKey::Divide)), // numpad
        ("VK_MULTIPLY", KeyCode::ControlKey(ControlKey::Multiply)), // numpad
        ("VK_SUBTRACT", KeyCode::ControlKey(ControlKey::Subtract)), // numpad
        ("VK_ADD", KeyCode::ControlKey(ControlKey::Add)), // numpad
        ("VK_DECIMAL", KeyCode::ControlKey(ControlKey::Decimal)), // numpad
        ("VK_F1", KeyCode::ControlKey(ControlKey::F1)),
        ("VK_F2", KeyCode::ControlKey(ControlKey::F2)),
        ("VK_F3", KeyCode::ControlKey(ControlKey::F3)),
        ("VK_F4", KeyCode::ControlKey(ControlKey::F4)),
        ("VK_F5", KeyCode::ControlKey(ControlKey::F5)),
        ("VK_F6", KeyCode::ControlKey(ControlKey::F6)),
        ("VK_F7", KeyCode::ControlKey(ControlKey::F7)),
        ("VK_F8", KeyCode::ControlKey(ControlKey::F8)),
        ("VK_F9", KeyCode::ControlKey(ControlKey::F9)),
        ("VK_F10", KeyCode::ControlKey(ControlKey::F10)),
        ("VK_F11", KeyCode::ControlKey(ControlKey::F11)),
        ("VK_F12", KeyCode::ControlKey(ControlKey::F12)),
        ("VK_ENTER", KeyCode::ControlKey(ControlKey::Return)),
        ("VK_CANCEL", KeyCode::ControlKey(ControlKey::Cancel)),
        ("VK_BACK", KeyCode::ControlKey(ControlKey::Backspace)),
        ("VK_TAB", KeyCode::ControlKey(ControlKey::Tab)),
        ("VK_CLEAR", KeyCode::ControlKey(ControlKey::Clear)),
        ("VK_RETURN", KeyCode::ControlKey(ControlKey::Return)),
        ("VK_SHIFT", KeyCode::ControlKey(ControlKey::Shift)),
        ("VK_CONTROL", KeyCode::ControlKey(ControlKey::Control)),
        ("VK_MENU", KeyCode::ControlKey(ControlKey::Alt)),
        ("VK_PAUSE", KeyCode::ControlKey(ControlKey::Pause)),
        ("VK_CAPITAL", KeyCode::ControlKey(ControlKey::CapsLock)),
        ("VK_KANA", KeyCode::ControlKey(ControlKey::Kana)),
        ("VK_HANGUL", KeyCode::ControlKey(ControlKey::Hangul)),
        ("VK_JUNJA", KeyCode::ControlKey(ControlKey::Junja)),
        ("VK_FINAL", KeyCode::ControlKey(ControlKey::Final)),
        ("VK_HANJA", KeyCode::ControlKey(ControlKey::Hanja)),
        ("VK_KANJI", KeyCode::ControlKey(ControlKey::Kanji)),
        ("VK_ESCAPE", KeyCode::ControlKey(ControlKey::Escape)),
        ("VK_CONVERT", KeyCode::ControlKey(ControlKey::Convert)),
        ("VK_SPACE", KeyCode::ControlKey(ControlKey::Space)),
        ("VK_PRIOR", KeyCode::ControlKey(ControlKey::PageUp)),
        ("VK_NEXT", KeyCode::ControlKey(ControlKey::PageDown)),
        ("VK_END", KeyCode::ControlKey(ControlKey::End)),
        ("VK_HOME", KeyCode::ControlKey(ControlKey::Home)),
        ("VK_LEFT", KeyCode::ControlKey(ControlKey::LeftArrow)),
        ("VK_UP", KeyCode::ControlKey(ControlKey::UpArrow)),
        ("VK_RIGHT", KeyCode::ControlKey(ControlKey::RightArrow)),
        ("VK_DOWN", KeyCode::ControlKey(ControlKey::DownArrow)),
        ("VK_SELECT", KeyCode::ControlKey(ControlKey::Select)),
        ("VK_PRINT", KeyCode::ControlKey(ControlKey::Print)),
        ("VK_EXECUTE", KeyCode::ControlKey(ControlKey::Execute)),
        ("VK_SNAPSHOT", KeyCode::ControlKey(ControlKey::Snapshot)),
        ("VK_INSERT", KeyCode::ControlKey(ControlKey::Insert)),
        ("VK_DELETE", KeyCode::ControlKey(ControlKey::Delete)),
        ("VK_HELP", KeyCode::ControlKey(ControlKey::Help)),
        ("VK_SLEEP", KeyCode::ControlKey(ControlKey::Sleep)),
        ("VK_SEPARATOR", KeyCode::ControlKey(ControlKey::Separator)),
        ("VK_NUMPAD0", KeyCode::ControlKey(ControlKey::Numpad0)),
        ("VK_NUMPAD1", KeyCode::ControlKey(ControlKey::Numpad1)),
        ("VK_NUMPAD2", KeyCode::ControlKey(ControlKey::Numpad2)),
        ("VK_NUMPAD3", KeyCode::ControlKey(ControlKey::Numpad3)),
        ("VK_NUMPAD4", KeyCode::ControlKey(ControlKey::Numpad4)),
        ("VK_NUMPAD5", KeyCode::ControlKey(ControlKey::Numpad5)),
        ("VK_NUMPAD6", KeyCode::ControlKey(ControlKey::Numpad6)),
        ("VK_NUMPAD7", KeyCode::ControlKey(ControlKey::Numpad7)),
        ("VK_NUMPAD8", KeyCode::ControlKey(ControlKey::Numpad8)),
        ("VK_NUMPAD9", KeyCode::ControlKey(ControlKey::Numpad9)),
        ("Apps", KeyCode::ControlKey(ControlKey::Apps)),
        ("Meta", KeyCode::ControlKey(ControlKey::Meta)),
        ("RAlt", KeyCode::ControlKey(ControlKey::RAlt)),
        ("RWin", KeyCode::ControlKey(ControlKey::RWin)),
        ("RControl", KeyCode::ControlKey(ControlKey::RControl)),
        ("RShift", KeyCode::ControlKey(ControlKey::RShift)),
        ("CTRL_ALT_DEL", KeyCode::ControlKey(ControlKey::CtrlAltDel)),
        ("LOCK_SCREEN", KeyCode::ControlKey(ControlKey::LockScreen)),
    ].iter().cloned().collect();
}
