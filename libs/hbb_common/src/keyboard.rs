use crate::{
    message_proto::{ControlKey, KeyEvent},
    protos::message::KeyboardMode,
};
use protobuf::Message;
use std::{
    convert::{TryFrom, TryInto},
    fmt::{self},
    slice::Iter,
    str::FromStr,
};

impl fmt::Display for KeyboardMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyboardMode::Legacy => write!(f, "legacy"),
            KeyboardMode::Map => write!(f, "map"),
            KeyboardMode::Translate => write!(f, "translate"),
            KeyboardMode::Auto => write!(f, "auto"),
        }
    }
}

impl FromStr for KeyboardMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "legacy" => Ok(KeyboardMode::Legacy),
            "map" => Ok(KeyboardMode::Map),
            "translate" => Ok(KeyboardMode::Translate),
            "auto" => Ok(KeyboardMode::Auto),
            _ => Err(()),
        }
    }
}

impl KeyboardMode {
    pub fn iter() -> Iter<'static, KeyboardMode> {
        static KEYBOARD_MODES: [KeyboardMode; 4] = [
            KeyboardMode::Legacy,
            KeyboardMode::Map,
            KeyboardMode::Translate,
            KeyboardMode::Auto,
        ];
        KEYBOARD_MODES.iter()
    }
}

impl TryInto<Vec<u8>> for KeyEvent {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        self.write_to_bytes()
            .map_err(|err| anyhow::anyhow!("Faild to encode key_event: {:?}", err))
    }
}

impl TryFrom<Vec<u8>> for KeyEvent {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        KeyEvent::parse_from_bytes(&value)
            .map_err(|err| anyhow::anyhow!("Faild to decode key_event: {:?}", err))
    }
}

impl ControlKey {
    pub fn swap_modifier(self) -> Self {
        match self {
            ControlKey::Control => ControlKey::Meta,
            ControlKey::Meta => ControlKey::Control,
            ControlKey::RControl => ControlKey::Meta,
            ControlKey::RWin => ControlKey::Control,
            _ => self,
        }
    }

    pub fn is_modifier(self) -> bool {
        matches!(
            self,
            ControlKey::Control
                | ControlKey::Meta
                | ControlKey::Alt
                | ControlKey::Shift
                | ControlKey::RControl
                | ControlKey::RWin
                | ControlKey::RAlt
                | ControlKey::RShift
        )
    }
}
