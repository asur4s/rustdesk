use bitflags::*;
use hbb_common::anyhow;
use hbb_common::message_proto::{key_event, ControlKey, KeyEvent};
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::{collections::HashMap, fmt::Display};

use crate::keyboard::get_peer_platform;

/// convert chr in KeyEvent
/// refs: <https://github.com/rustdesk/rustdesk/blob/5ebaefe08a66459f6fda8d94409d9a734f6e6bdc/libs/hbb_common/protos/message.proto#L205>
#[derive(Debug, Clone)]
pub struct CharCode {
    value: u32,
}

#[allow(dead_code)]
impl CharCode {
    pub fn new(position_code: u32, sys_code: u32) -> CharCode {
        let value = (position_code & 0x0000FFFF) | ((sys_code as u32) << 16);
        CharCode { value }
    }

    pub fn from_u32(value: u32) -> CharCode {
        CharCode { value }
    }

    pub fn get_pos_code(&self) -> u32 {
        self.value & 0x0000FFFF
    }

    pub fn get_sys_code(&self) -> u32 {
        self.value >> 16
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Display for CharCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(PosCode={:?}, SysCode={:?})",
            self.get_pos_code(),
            self.get_sys_code()
        )
    }
}
/// Keycode for key events.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyCode {
    ControlKey(ControlKey),
    Chr(u32),
    Raw(u32),
    Seq(String),
    PhysCode(Key),
}

pub trait KeyOps {
    fn pos_code(&self);
    fn sys_code(&self);
    fn from_pos(code: u32) -> Option<Key>;
    fn from_event(event: &Event) -> Option<Key>;
}

pub trait KeyConvert {
    fn control_key(&self) -> Option<ControlKey>;
    fn swap_modifier(self) -> Key;
}

impl KeyConvert for Key {
    fn swap_modifier(self) -> Key {
        match self {
            Key::ControlLeft => Key::MetaLeft,
            Key::MetaLeft => Key::ControlLeft,
            Key::ControlRight => Key::MetaRight,
            Key::MetaRight => Key::ControlRight,
            _ => self,
        }
    }
    fn control_key(&self) -> Option<ControlKey> {
        match self {
            Key::Alt => Some(ControlKey::Alt),
            Key::AltGr => Some(ControlKey::RAlt),
            Key::Backspace => Some(ControlKey::Backspace),
            Key::ControlLeft => Some(ControlKey::Control),
            Key::ControlRight => Some(ControlKey::RControl),
            Key::DownArrow => Some(ControlKey::DownArrow),
            Key::Escape => Some(ControlKey::Escape),
            Key::F1 => Some(ControlKey::F1),
            Key::F10 => Some(ControlKey::F10),
            Key::F11 => Some(ControlKey::F11),
            Key::F12 => Some(ControlKey::F12),
            Key::F2 => Some(ControlKey::F2),
            Key::F3 => Some(ControlKey::F3),
            Key::F4 => Some(ControlKey::F4),
            Key::F5 => Some(ControlKey::F5),
            Key::F6 => Some(ControlKey::F6),
            Key::F7 => Some(ControlKey::F7),
            Key::F8 => Some(ControlKey::F8),
            Key::F9 => Some(ControlKey::F9),
            Key::LeftArrow => Some(ControlKey::LeftArrow),
            Key::MetaLeft => Some(ControlKey::Meta),
            Key::MetaRight => Some(ControlKey::RWin),
            Key::Return => Some(ControlKey::Return),
            Key::RightArrow => Some(ControlKey::RightArrow),
            Key::ShiftLeft => Some(ControlKey::Shift),
            Key::ShiftRight => Some(ControlKey::RShift),
            Key::Space => Some(ControlKey::Space),
            Key::Tab => Some(ControlKey::Tab),
            Key::UpArrow => Some(ControlKey::UpArrow),
            Key::Delete => Some(ControlKey::Delete),
            Key::Apps => Some(ControlKey::Apps), // Menu
            Key::Cancel => Some(ControlKey::Cancel),
            Key::Clear => Some(ControlKey::Clear),
            Key::Kana => Some(ControlKey::Kana),
            Key::Hangul => Some(ControlKey::Hangul),
            Key::Junja => Some(ControlKey::Junja),
            Key::Final => Some(ControlKey::Final),
            Key::Hanja => Some(ControlKey::Hanja),
            Key::Hanji => Some(ControlKey::Hanja),
            Key::Convert => Some(ControlKey::Convert),
            Key::Print => Some(ControlKey::Print),
            Key::Select => Some(ControlKey::Select),
            Key::Execute => Some(ControlKey::Execute),
            Key::PrintScreen => Some(ControlKey::Snapshot),
            Key::Help => Some(ControlKey::Help),
            Key::Sleep => Some(ControlKey::Sleep),
            Key::Separator => Some(ControlKey::Separator),
            Key::KpReturn => Some(ControlKey::NumpadEnter),
            Key::Kp0 => Some(ControlKey::Numpad0),
            Key::Kp1 => Some(ControlKey::Numpad1),
            Key::Kp2 => Some(ControlKey::Numpad2),
            Key::Kp3 => Some(ControlKey::Numpad3),
            Key::Kp4 => Some(ControlKey::Numpad4),
            Key::Kp5 => Some(ControlKey::Numpad5),
            Key::Kp6 => Some(ControlKey::Numpad6),
            Key::Kp7 => Some(ControlKey::Numpad7),
            Key::Kp8 => Some(ControlKey::Numpad8),
            Key::Kp9 => Some(ControlKey::Numpad9),
            Key::KpDivide => Some(ControlKey::Divide),
            Key::KpMultiply => Some(ControlKey::Multiply),
            Key::KpDecimal => Some(ControlKey::Decimal),
            Key::KpMinus => Some(ControlKey::Subtract),
            Key::KpPlus => Some(ControlKey::Add),
            Key::CapsLock => Some(ControlKey::CapsLock),
            Key::NumLock => Some(ControlKey::NumLock),
            Key::ScrollLock => Some(ControlKey::Scroll),
            Key::Home => Some(ControlKey::Home),
            Key::End => Some(ControlKey::End),
            Key::Insert => Some(ControlKey::Insert),
            Key::PageUp => Some(ControlKey::PageUp),
            Key::PageDown => Some(ControlKey::PageDown),
            Key::Pause => Some(ControlKey::Pause),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawKeyboardEvent {
    /// The physical location of the key on an ANSI-Standard US layout
    pub key: Key,
    pub press: bool,
    pub modifiers: Modifiers,
    /// The OS and hardware dependent key code for the key
    /// - windows: virtual key
    /// - linux: keysym
    pub sys_code: u32,
    /// The *other* OS and hardware dependent key code for the key
    pub pos_code: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    /// Which key was pressed
    pub keycode: KeyCode,
    // pressed or release
    pub press: bool,
    /// Which modifiers are down
    pub modifiers: Modifiers,
    pub raw_event: Option<RawKeyboardEvent>,
    pub remote_code: Option<RemoteCode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteCode {
    pub cur_platform: String,
    pub peer_platform: String,
    pub cur_sys_code: u32,
    pub remote_pos_code: u32,
}
#[allow(dead_code)]
impl KeyboardEvent {
    pub fn from_events(event: &Event, key_event: &KeyEvent) -> anyhow::Result<Self> {
        let (key, press) = match event.event_type {
            EventType::KeyPress(key) => (key, true),
            EventType::KeyRelease(key) => (key, false),
            _ => anyhow::bail!("Unexcept Event type"),
        };
        // todo:
        let modifiers = Modifiers::NONE;
        let raw_event = RawKeyboardEvent {
            key,
            press,
            modifiers,
            // FIXME: get keysym in linux
            sys_code: event.code as u32,
            pos_code: event.scan_code,
        };

        let mut remote_code = None;
        let keycode = if let Some(union) = key_event.union.clone() {
            match union {
                key_event::Union::ControlKey(key) => {
                    let control_key = key
                        .enum_value()
                        .map_err(|err| anyhow::anyhow!("Failed to get enum value: {:?}", err))?;
                    KeyCode::ControlKey(control_key)
                }
                key_event::Union::Chr(chr) => {
                    let char_code = CharCode::from_u32(chr);
                    let _key = Key::from_pos(char_code.get_pos_code());

                    remote_code = Some(RemoteCode {
                        cur_platform: whoami::platform().to_string(),
                        peer_platform: get_peer_platform(),
                        cur_sys_code: char_code.get_sys_code(),
                        remote_pos_code: char_code.get_pos_code(),
                    });

                    let key = Key::from_pos(char_code.get_pos_code());

                    if let Some(key) = key {
                        if !press {
                            KeyCode::PhysCode(key)
                        } else if let Some(control_key) = key.control_key() {
                            KeyCode::ControlKey(control_key)
                        } else {
                            KeyCode::Raw(char_code.get_sys_code())
                        }
                    } else {
                        KeyCode::Raw(char_code.get_sys_code())
                    }
                }
                key_event::Union::Unicode(uni) => KeyCode::Chr(uni),
                key_event::Union::Seq(s) => KeyCode::Seq(s),
                _ => anyhow::bail!("Unexcept KeyEvent type"),
            }
        } else {
            KeyCode::PhysCode(key)
        };

        Ok(KeyboardEvent {
            keycode,
            press,
            modifiers,
            raw_event: Some(raw_event),
            remote_code,
        })
    }
}

bitflags! {
    ///! https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    ///! Use xmodmap -pm to get meaning of modifier
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

#[test]
fn test_chr_code() {
    // Left Shift: win -> linux: pos_code=50, sys_code=160
    let chr_code = CharCode::from_u32(10485810);
    assert_eq!(50, chr_code.get_pos_code());
    assert_eq!(160, chr_code.get_sys_code());
}
