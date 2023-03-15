use crate::keyboard_impl::KeyCode;
use hbb_common::{
    lazy_static,
    message_proto::{ControlKey, PhysKeyCode},
};
use rdev::Key;
use std::collections::HashMap;
use std::hash::Hash;

pub fn convert_phys_to_key(phys: &PhysKeyCode) -> Option<Key> {
    PHYS_TO_KEY.get(phys).cloned()
}

pub fn convert_key_to_phys(key: &Key) -> Option<PhysKeyCode> {
    KEY_TO_PHYS.get(key).cloned()
}

pub fn convert_control_key_to_key(phys: &ControlKey) -> Option<Key> {
    CTRLKEY_TO_KEY.get(phys).cloned()
}

pub fn convert_key_to_control_key(key: &Key) -> Option<ControlKey> {
    KEY_TO_CTRLKEY.get(key).cloned()
}

lazy_static::lazy_static! {
    static ref CTRLKEY_TO_KEY: HashMap<ControlKey, Key> = {
        if let MapType::Forward(map) = build_map(&CTRLKEY_KEY, false) {
            map
        } else {
            HashMap::new()
        }
    };
    static ref KEY_TO_CTRLKEY: HashMap<Key, ControlKey> = {
        if let MapType::Reverse(map) = build_map(&CTRLKEY_KEY, true) {
            map
        } else {
            HashMap::new()
        }
    };
     static ref PHYS_TO_KEY: HashMap<PhysKeyCode, Key> = {
        if let MapType::Forward(map) = build_map(&PHYS_KEY, false) {
            map
        } else {
            HashMap::new()
        }
    };
     static ref KEY_TO_PHYS: HashMap<Key, PhysKeyCode> = {
        if let MapType::Reverse(map) = build_map(&PHYS_KEY, true) {
            map
        } else {
            HashMap::new()
        }
    };
}

enum MapType<K, V> {
    Forward(HashMap<K, V>),
    Reverse(HashMap<V, K>),
}

fn build_map<K, V>(key_values: &[(K, V)], reverse: bool) -> MapType<K, V>
where
    K: Eq + Hash + Copy,
    V: Eq + Hash + Copy,
{
    if !reverse {
        let mut map = HashMap::new();
        for (key, value) in key_values {
            map.insert(key.clone(), value.clone());
        }
        MapType::Forward(map)
    } else {
        let mut map = HashMap::new();
        for (key, value) in key_values {
            map.insert(value.clone(), key.clone());
        }
        MapType::Reverse(map)
    }
}

lazy_static::lazy_static! {
    static ref CTRLKEY_KEY: Vec<(ControlKey, Key)> = vec![
        (ControlKey::Alt , Key::Alt),
        (ControlKey::RAlt , Key::AltGr),
        (ControlKey::Backspace , Key::Backspace),
        (ControlKey::Control , Key::ControlLeft),
        (ControlKey::RControl , Key::ControlRight),
        (ControlKey::DownArrow , Key::DownArrow),
        (ControlKey::Escape , Key::Escape),
        (ControlKey::F1 , Key::F1),
        (ControlKey::F10 , Key::F10),
        (ControlKey::F11 , Key::F11),
        (ControlKey::F12 , Key::F12),
        (ControlKey::F2 , Key::F2),
        (ControlKey::F3 , Key::F3),
        (ControlKey::F4 , Key::F4),
        (ControlKey::F5 , Key::F5),
        (ControlKey::F6 , Key::F6),
        (ControlKey::F7 , Key::F7),
        (ControlKey::F8 , Key::F8),
        (ControlKey::F9 , Key::F9),
        (ControlKey::LeftArrow , Key::LeftArrow),
        (ControlKey::Meta , Key::MetaLeft),
        (ControlKey::RWin , Key::MetaRight),
        (ControlKey::Return , Key::Return),
        (ControlKey::RightArrow , Key::RightArrow),
        (ControlKey::Shift , Key::ShiftLeft),
        (ControlKey::RShift , Key::ShiftRight),
        (ControlKey::Space , Key::Space),
        (ControlKey::Tab , Key::Tab),
        (ControlKey::UpArrow , Key::UpArrow),
        (ControlKey::Delete , Key::Delete),
        (ControlKey::Apps , Key::Apps), // Menu
        (ControlKey::Cancel , Key::Cancel),
        (ControlKey::Clear , Key::Clear),
        (ControlKey::Kana , Key::Kana),
        (ControlKey::Hangul , Key::Hangul),
        (ControlKey::Junja , Key::Junja),
        (ControlKey::Final , Key::Final),
        (ControlKey::Hanja , Key::Hanja),
        (ControlKey::Hanja , Key::Hanji),
        (ControlKey::Convert , Key::Convert),
        (ControlKey::Print , Key::Print),
        (ControlKey::Select , Key::Select),
        (ControlKey::Execute , Key::Execute),
        (ControlKey::Snapshot , Key::PrintScreen),
        (ControlKey::Help , Key::Help),
        (ControlKey::Sleep , Key::Sleep),
        (ControlKey::Separator , Key::Separator),
        (ControlKey::NumpadEnter , Key::KpReturn),
        (ControlKey::Numpad0 , Key::Kp0),
        (ControlKey::Numpad1 , Key::Kp1),
        (ControlKey::Numpad2 , Key::Kp2),
        (ControlKey::Numpad3 , Key::Kp3),
        (ControlKey::Numpad4 , Key::Kp4),
        (ControlKey::Numpad5 , Key::Kp5),
        (ControlKey::Numpad6 , Key::Kp6),
        (ControlKey::Numpad7 , Key::Kp7),
        (ControlKey::Numpad8 , Key::Kp8),
        (ControlKey::Numpad9 , Key::Kp9),
        (ControlKey::Divide , Key::KpDivide),
        (ControlKey::Multiply , Key::KpMultiply),
        (ControlKey::Decimal , Key::KpDecimal),
        (ControlKey::Subtract , Key::KpMinus),
        (ControlKey::Add , Key::KpPlus),
        (ControlKey::CapsLock , Key::CapsLock),
        (ControlKey::NumLock , Key::NumLock),
        (ControlKey::Scroll , Key::ScrollLock),
        (ControlKey::Home , Key::Home),
        (ControlKey::End , Key::End),
        (ControlKey::Insert , Key::Insert),
        (ControlKey::PageUp , Key::PageUp),
        (ControlKey::PageDown , Key::PageDown),
        (ControlKey::Pause , Key::Pause),
    ];
    static ref PHYS_KEY: Vec<(PhysKeyCode, Key)> = vec![
        (PhysKeyCode::AltLeft, Key::Alt,),
        (PhysKeyCode::AltRight, Key::AltGr,),
        (PhysKeyCode::ControlLeft, Key::ControlLeft,),
        (PhysKeyCode::ControlRight, Key::ControlRight,),
        (PhysKeyCode::Backspace, Key::Backspace,),
        (PhysKeyCode::CapsLock, Key::CapsLock,),
        (PhysKeyCode::Delete, Key::Delete,),
        (PhysKeyCode::DownArrow, Key::Delete,),
        (PhysKeyCode::End, Key::End,),
        (PhysKeyCode::Escape, Key::Escape,),
        (PhysKeyCode::F1, Key::F1,),
        (PhysKeyCode::F3, Key::F3,),
        (PhysKeyCode::F4, Key::F4,),
        (PhysKeyCode::F5, Key::F5,),
        (PhysKeyCode::F6, Key::F6,),
        (PhysKeyCode::F7, Key::F7,),
        (PhysKeyCode::F8, Key::F8,),
        (PhysKeyCode::F9, Key::F9,),
        (PhysKeyCode::F10, Key::F10,),
        (PhysKeyCode::F11, Key::F11,),
        (PhysKeyCode::F12, Key::F12,),
        (PhysKeyCode::F13, Key::F13,),
        (PhysKeyCode::F14, Key::F14,),
        (PhysKeyCode::F15, Key::F15,),
        (PhysKeyCode::F16, Key::F16,),
        (PhysKeyCode::F17, Key::F17,),
        (PhysKeyCode::F18, Key::F18,),
        (PhysKeyCode::F19, Key::F19,),
        (PhysKeyCode::F2, Key::F2,),
        (PhysKeyCode::F20, Key::F20,),
        (PhysKeyCode::Home, Key::Home,),
        (PhysKeyCode::LeftArrow, Key::LeftArrow,),
        (PhysKeyCode::MetaLeft, Key::MetaLeft,),
        (PhysKeyCode::MetaRight, Key::MetaRight,),
        (PhysKeyCode::PageDown, Key::PageDown,),
        (PhysKeyCode::PageUp, Key::PageUp,),
        (PhysKeyCode::Return, Key::Return,),
        (PhysKeyCode::ShiftLeft, Key::ShiftLeft,),
        (PhysKeyCode::ShiftRight, Key::ShiftRight,),
        (PhysKeyCode::Space, Key::Space,),
        (PhysKeyCode::Tab, Key::Tab,),
        (PhysKeyCode::UpArrow, Key::UpArrow,),
        (PhysKeyCode::PrintScreen, Key::PrintScreen,),
        (PhysKeyCode::ScrollLock, Key::ScrollLock,),
        (PhysKeyCode::Pause, Key::Pause,),
        (PhysKeyCode::NumLock, Key::NumLock,),
        (PhysKeyCode::BackQuote, Key::BackQuote,),
        (PhysKeyCode::Num1, Key::Num1,),
        (PhysKeyCode::Num2, Key::Num2,),
        (PhysKeyCode::Num3, Key::Num3,),
        (PhysKeyCode::Num4, Key::Num4,),
        (PhysKeyCode::Num5, Key::Num5,),
        (PhysKeyCode::Num6, Key::Num6,),
        (PhysKeyCode::Num7, Key::Num7,),
        (PhysKeyCode::Num8, Key::Num8,),
        (PhysKeyCode::Num9, Key::Num9,),
        (PhysKeyCode::Num0, Key::Num0,),
        (PhysKeyCode::Minus, Key::Minus,),
        (PhysKeyCode::Equal, Key::Equal,),
        (PhysKeyCode::KeyQ, Key::KeyQ,),
        (PhysKeyCode::KeyW, Key::KeyW,),
        (PhysKeyCode::KeyE, Key::KeyE,),
        (PhysKeyCode::KeyR, Key::KeyR,),
        (PhysKeyCode::KeyT, Key::KeyT,),
        (PhysKeyCode::KeyY, Key::KeyY,),
        (PhysKeyCode::KeyU, Key::KeyU,),
        (PhysKeyCode::KeyI, Key::KeyI,),
        (PhysKeyCode::KeyO, Key::KeyO,),
        (PhysKeyCode::KeyP, Key::KeyP,),
        (PhysKeyCode::LeftBracket, Key::LeftBracket,),
        (PhysKeyCode::RightBracket, Key::RightBracket,),
        (PhysKeyCode::KeyA, Key::KeyA,),
        (PhysKeyCode::KeyS, Key::KeyS,),
        (PhysKeyCode::KeyD, Key::KeyD,),
        (PhysKeyCode::KeyF, Key::KeyF,),
        (PhysKeyCode::KeyG, Key::KeyG,),
        (PhysKeyCode::KeyH, Key::KeyH,),
        (PhysKeyCode::KeyJ, Key::KeyJ,),
        (PhysKeyCode::KeyK, Key::KeyK,),
        (PhysKeyCode::KeyL, Key::KeyL,),
        (PhysKeyCode::SemiColon, Key::SemiColon,),
        (PhysKeyCode::Quote, Key::Quote,),
        (PhysKeyCode::BackSlash, Key::BackSlash,),
        (PhysKeyCode::IntlBackslash, Key::IntlBackslash,),
        (PhysKeyCode::KeyZ, Key::KeyZ,),
        (PhysKeyCode::KeyX, Key::KeyX,),
        (PhysKeyCode::KeyC, Key::KeyC,),
        (PhysKeyCode::KeyV, Key::KeyV,),
        (PhysKeyCode::KeyB, Key::KeyB,),
        (PhysKeyCode::KeyN, Key::KeyN,),
        (PhysKeyCode::KeyM, Key::KeyM,),
        (PhysKeyCode::Comma, Key::Comma,),
        (PhysKeyCode::Dot, Key::Dot,),
        (PhysKeyCode::Slash, Key::Slash,),
        (PhysKeyCode::Insert, Key::Insert,),
        (PhysKeyCode::KpReturn, Key::KpReturn,),
        (PhysKeyCode::KpMinus, Key::KpMinus,),
        (PhysKeyCode::KpPlus, Key::KpPlus,),
        (PhysKeyCode::KpMultiply, Key::KpMultiply,),
        (PhysKeyCode::KpDivide, Key::KpDivide,),
        (PhysKeyCode::KpDecimal, Key::KpDecimal,),
        (PhysKeyCode::Kp0, Key::Kp0,),
        (PhysKeyCode::Kp1, Key::Kp1,),
        (PhysKeyCode::Kp2, Key::Kp2,),
        (PhysKeyCode::Kp3, Key::Kp3,),
        (PhysKeyCode::Kp4, Key::Kp4,),
        (PhysKeyCode::Kp5, Key::Kp5,),
        (PhysKeyCode::Kp6, Key::Kp6,),
        (PhysKeyCode::Kp7, Key::Kp7,),
        (PhysKeyCode::Kp8, Key::Kp8,),
        (PhysKeyCode::Kp9, Key::Kp9,),
        (PhysKeyCode::Function, Key::Function,),
        (PhysKeyCode::Help, Key::Help,),
        (PhysKeyCode::RightArrow, Key::RightArrow,),
        (PhysKeyCode::KpDelete, Key::Delete,),
        (PhysKeyCode::VolumeDown, Key::VolumeDown,),
        (PhysKeyCode::VolumeUp, Key::VolumeUp,),
        (PhysKeyCode::VolumeMute, Key::VolumeMute,),
        (PhysKeyCode::Menu, Key::Apps,),
    ];
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
fn test_phys_to_key() {
    assert_eq!(PHYS_TO_KEY.get(&PhysKeyCode::AltLeft), Some(&Key::Alt));
}
