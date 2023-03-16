use super::{convert_key_to_control_key, convert_key_to_phys, convert_phys_to_key};
use crate::client::get_key_state;
use crate::keyboard_impl::is_altgr_pressed;
use bitflags::*;
use hbb_common::message_proto::{key_event, ControlKey, KeyEvent, PhysKeyCode, RawKeyEvent};
use hbb_common::{anyhow, ResultType};
use rdev::{Event, EventType, Key};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ptr::null_mut;
use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
pub const TRUE: i32 = 1;
#[allow(dead_code)]
pub const FALSE: i32 = 0;

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
    PhysCode(PhysKeyCode),
}

impl KeyCode {
    pub fn from_key_event(key_event: &KeyEvent) -> anyhow::Result<KeyCode> {
        let keycode = if let Some(union) = key_event.union.clone() {
            match union {
                key_event::Union::ControlKey(key) => {
                    let control_key = key
                        .enum_value()
                        .map_err(|err| anyhow::anyhow!("Failed to get enum value: {:?}", err))?;
                    KeyCode::ControlKey(control_key)
                }
                /// Chr is used for other events(maybe not real char).
                key_event::Union::Chr(chr) => KeyCode::Raw(chr),
                key_event::Union::Unicode(uni) => KeyCode::Chr(uni),
                key_event::Union::Seq(s) => KeyCode::Seq(s),
                _ => anyhow::bail!("Unexcept KeyEvent type"),
            }
        } else {
            anyhow::bail!("Unexcept KeyEvent type")
        };
        Ok(keycode)
    }
}

impl ToString for KeyCode {
    fn to_string(&self) -> String {
        match self {
            KeyCode::ControlKey(control_key) => format!("{:?}", control_key),
            KeyCode::Chr(chr) => {
                if let Some(chr) = char::from_u32(*chr) {
                    format!("Chr({:?})", chr)
                } else {
                    format!("None",)
                }
            }
            KeyCode::Raw(raw) => format!("Raw({})", raw),
            KeyCode::Seq(seq) => format!("Seq({:?})", seq),
            KeyCode::PhysCode(phys) => format!("Phys({:?})", phys),
        }
    }
}

pub trait KeyOps {
    /// - Windows: scancode
    /// - Linux: keycode
    /// - MacOS: Keycode
    fn pos_code(&self) -> ResultType<u32>;
    /// - Windows: Virtual Key
    /// - Linux: Keysym
    /// - MacOS: Keycode
    fn sys_code(&self) -> ResultType<u32>;
    fn from_pos(code: u32) -> ResultType<Key>;
    /// Get Key base on Physical location
    fn from_event(event: &Event) -> ResultType<Key>;
}

pub trait KeyEventOps {
    fn from_event(event: &Event) -> ResultType<KeyEvent>;
    fn format(&self) -> String;
}

impl KeyEventOps for KeyEvent {
    fn from_event(event: &Event) -> ResultType<KeyEvent> {
        let mut key_event = KeyEvent::new();

        let key = Key::from_event(event)?;
        let phys = key.to_phys()?;
        let press = event.press();

        if let Some(unicode_info) = event.unicode.clone() {
            if !unicode_info.is_dead {
                if let Some(name) = unicode_info.name {
                    if name.len() == 1 {
                        let chr = name
                            .chars()
                            .next()
                            .ok_or(anyhow::anyhow!("Failed to get char"))?;
                        key_event.set_unicode(chr as u32)
                    } else if name.len() > 1 {
                        key_event.set_seq(name.to_string());
                    } else {
                        anyhow::bail!("Failed to get unicode name");
                    }
                }
            };
        }
        if key_event.union.is_none() {
            // TODO: handle VK_HANGUL in Korean
            if let Ok(control_key) = key.to_control_key() {
                key_event.set_control_key(control_key);
            }
        }

        let modifiers = {
            let mods = Modifiers::get_current_modifiers();
            if event.is_altgr() || is_altgr_pressed() {
                mods.normalize_altgr()
            } else {
                mods
            }
        };

        key_event.down = press;
        key_event.raw_event = Some(RawKeyEvent {
            phys: phys.into(),
            press: press,
            modifiers: modifiers.bits(),
            sys_code: key.sys_code()?,
            ..Default::default()
        })
        .into();

        println!("{}", &key_event.format());

        Ok(key_event)
    }

    fn format(&self) -> String {
        let phys = format!("{:?}", self.raw_event.phys);
        let keycode = if let Some(keycode) = KeyCode::from_key_event(self).ok() {
            keycode.to_string()
        } else {
            format!("None")
        };
        let modifiers = if let Some(modifiers) = Modifiers::from_bits(self.raw_event.modifiers) {
            modifiers.to_string()
        } else {
            format!("None")
        };
        format!(
            "PhysKeyCode={0: <12} | press={1: <5} | keycode={2: <12} | sys_code={3: <10} | modifiers={4:}",
            phys, self.raw_event.press, keycode, self.raw_event.sys_code, modifiers,
        )
    }
}

pub trait KeyConvert {
    fn swap_modifier(self) -> Key;
    fn from_phys(phys: &PhysKeyCode) -> ResultType<Key>;
    fn to_phys(&self) -> ResultType<PhysKeyCode>;
    fn to_control_key(&self) -> ResultType<ControlKey>;
}

impl KeyConvert for Key {
    fn from_phys(phys: &PhysKeyCode) -> ResultType<Key> {
        convert_phys_to_key(phys).ok_or(anyhow::anyhow!("Failed to convert {:?} to key", phys))
    }

    fn to_phys(&self) -> ResultType<PhysKeyCode> {
        convert_key_to_phys(self).ok_or(anyhow::anyhow!(
            "Failed to Convert {:?} to PhysKeyCode",
            self
        ))
    }

    fn swap_modifier(self) -> Key {
        match self {
            Key::ControlLeft => Key::MetaLeft,
            Key::MetaLeft => Key::ControlLeft,
            Key::ControlRight => Key::MetaRight,
            Key::MetaRight => Key::ControlRight,
            _ => self,
        }
    }
    fn to_control_key(&self) -> ResultType<ControlKey> {
        convert_key_to_control_key(self).ok_or(anyhow::anyhow!(
            "Failed to convert {:?} to ControlKey",
            self
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawKeyboardEvent {
    /// The physical location of the key on an ANSI-Standard US layout
    pub phys: PhysKeyCode,
    pub press: bool,
    pub modifiers: Modifiers,
    /// The OS and hardware dependent key code for the key
    /// - windows: virtual key
    /// - linux: keysym
    pub sys_code: u32,
}

impl RawKeyboardEvent {
    pub fn with_phys(phys: PhysKeyCode, press: bool) -> RawKeyboardEvent {
        RawKeyboardEvent {
            phys,
            press,
            modifiers: Modifiers::NONE,
            sys_code: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEvent {
    /// Which key was pressed
    pub keycode: Option<KeyCode>,
    // pressed or release
    pub press: bool,
    /// Which modifiers are down
    pub modifiers: Modifiers,
    pub raw_event: Option<RawKeyboardEvent>,
}

#[allow(dead_code)]
impl KeyboardEvent {
    pub fn from_events(event: &Event, key_event: &KeyEvent) -> anyhow::Result<Self> {
        let key = event.key()?;
        let press = event.press();
        let phys = key.to_phys()?;
        // FIXME by Chieh: get modifiers
        let modifiers = Modifiers::NONE;
        let raw_event = RawKeyboardEvent {
            phys,
            press,
            modifiers,
            // FIXME by Chieh: get keysym in linux
            sys_code: key.sys_code()?,
        };

        let keycode = KeyCode::from_key_event(key_event).ok();

        Ok(KeyboardEvent {
            keycode,
            press,
            modifiers,
            raw_event: Some(raw_event),
        })
    }

    pub fn from_key_event(key_event: &KeyEvent) -> anyhow::Result<Self> {
        let raw_event = Some(RawKeyboardEvent {
            phys: key_event.raw_event.phys.enum_value().map_err(|value| {
                anyhow::anyhow!("Unexcept raw event: faild to convert {:?} to Phys", value)
            })?,
            press: key_event.raw_event.press,
            modifiers: Modifiers::from_bits(key_event.raw_event.modifiers).ok_or(
                anyhow::anyhow!("Invalid modifiers: {:?}", key_event.raw_event.modifiers),
            )?,
            sys_code: key_event.raw_event.sys_code,
        });

        Ok(Self {
            keycode: KeyCode::from_key_event(key_event).ok(),
            press: key_event.press,
            // TODO:
            modifiers: Modifiers::NONE,
            raw_event,
        })
    }

    pub fn with_phys(phys: PhysKeyCode, press: bool) -> KeyboardEvent {
        KeyboardEvent {
            keycode: Some(KeyCode::PhysCode(phys)),
            press,
            modifiers: Modifiers::NONE,
            raw_event: None,
        }
    }
}

bitflags! {
    ///! https://stackoverflow.com/questions/69656145/how-does-modifiersas-in-xmodmap-work-under-linux-operating-system
    ///! Use xmodmap -pm to get meaning of modifier
    #[derive(Default, Deserialize, Serialize)]
    pub struct Modifiers: u32 {
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

impl Modifiers {
    pub fn is_shortcut(&self) -> bool {
        for mods in [
            Modifiers::CTRL,
            Modifiers::ALT,
            Modifiers::LEFT_CTRL,
            Modifiers::LEFT_ALT,
            Modifiers::RIGHT_CTRL,
            Modifiers::RIGHT_ALT,
            Modifiers::META,
        ] {
            if self.contains(mods) {
                return true;
            }
        }
        false
    }

    pub fn trans_positional_mods(self) -> Self {
        let mut modifiers = self;

        for (m, (left_mod, right_mod)) in [
            (Self::ALT, (Self::LEFT_ALT, Self::RIGHT_ALT)),
            (Self::CTRL, (Self::LEFT_CTRL, Self::RIGHT_CTRL)),
            (Self::SHIFT, (Self::LEFT_SHIFT, Self::RIGHT_SHIFT)),
        ] {
            if self.contains(left_mod) || self.contains(right_mod) {
                modifiers = modifiers - left_mod - right_mod;
                modifiers |= m;
            }
        }
        modifiers
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

impl ToString for Modifiers {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if *self == Self::NONE {
            s.push_str("NONE");
        }

        for (value, label) in [
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
            if !self.contains(value) {
                continue;
            }
            if !s.is_empty() {
                s.push('|');
            }
            s.push_str(label);
        }

        s
    }
}

pub trait EventInfo {
    fn key(&self) -> ResultType<Key>;
    fn press(&self) -> bool;
    fn is_altgr(&self) -> bool;
}

impl EventInfo for Event {
    fn key(&self) -> ResultType<Key> {
        match self.event_type {
            EventType::KeyPress(key) => Ok(key),
            EventType::KeyRelease(key) => Ok(key),
            _ => anyhow::bail!("Unexcepted Event"),
        }
    }

    fn press(&self) -> bool {
        match self.event_type {
            EventType::KeyPress(_) => true,
            EventType::KeyRelease(_) => false,
            _ => false,
        }
    }

    fn is_altgr(&self) -> bool {
        if self.scan_code == 57400 {
            true
        } else {
            false
        }
    }
}

#[test]
fn test_chr_code() {
    // Left Shift: win -> linux: pos_code=50, sys_code=160
    let chr_code = CharCode::from_u32(10485810);
    assert_eq!(50, chr_code.get_pos_code());
    assert_eq!(160, chr_code.get_sys_code());
}

#[test]
fn test_diff_modifiers() {
    let target_modifiers = Modifiers::LEFT_ALT;
    let modifiers = Modifiers::NONE;

    dbg!(modifiers.diff_modifiers(&target_modifiers));
}
