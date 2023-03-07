use super::xdo::EnigoXdo;
use crate::{Key, KeyboardControllable, MouseButton, MouseControllable};

use keyboarder::{
    connection::ConnectionOps,
    platform_impl::{Connection, Simulator},
};
use std::io::Read;
use tfc::{traits::*, Context as TFC_Context, Key as TFC_Key};
pub type CustomKeyboard = Box<dyn KeyboardControllable + Send>;
pub type CustomMouce = Box<dyn MouseControllable + Send>;

/// The main struct for handling the event emitting
// #[derive(Default)]
pub struct Enigo {
    xdo: EnigoXdo,
    is_x11: bool,
    tfc: Option<TFC_Context>,
    keyboarder: Option<Simulator>,
    custom_keyboard: Option<CustomKeyboard>,
    custom_mouse: Option<CustomMouce>,
}

impl Enigo {
    /// Get delay of xdo implementation.
    pub fn delay(&self) -> u64 {
        self.xdo.delay()
    }
    /// Set delay of xdo implementation.
    pub fn set_delay(&mut self, delay: u64) {
        self.xdo.set_delay(delay)
    }
    /// Set custom keyboard.
    pub fn set_custom_keyboard(&mut self, custom_keyboard: CustomKeyboard) {
        self.custom_keyboard = Some(custom_keyboard)
    }
    /// Set custom mouse.
    pub fn set_custom_mouse(&mut self, custom_mouse: CustomMouce) {
        self.custom_mouse = Some(custom_mouse)
    }
    /// Get custom keyboard.
    pub fn get_custom_keyboard(&mut self) -> &mut Option<CustomKeyboard> {
        &mut self.custom_keyboard
    }
    /// Get custom mouse.
    pub fn get_custom_mouse(&mut self) -> &mut Option<CustomMouce> {
        &mut self.custom_mouse
    }

    fn simulate_char_by_tfc(tfc: &mut TFC_Context, chr: char, down: bool) -> anyhow::Result<()> {
        let res = if down {
            tfc.unicode_char_down(chr)
        } else {
            tfc.unicode_char_up(chr)
        };
        res.map_err(|err| anyhow::anyhow!("Failed to simulate: {:?}", err))
    }

    fn simulate_key_by_tfc(tfc: &mut TFC_Context, key: Key, down: bool) -> anyhow::Result<()> {
        if let Some(key) = convert_to_tfc_key(key) {
            let res = if down {
                tfc.key_down(key)
            } else {
                tfc.key_up(key)
            };
            res.map_err(|err| anyhow::anyhow!("Failed to simulate: {:?}", err))
        } else {
            anyhow::bail!("Not found key about TFC: {:?}", key)
        }
    }

    fn simulate_by_tfc(&mut self, key: Key, down: bool) -> anyhow::Result<()> {
        if let Some(tfc) = &mut self.tfc {
            match key {
                Key::Layout(chr) => Self::simulate_char_by_tfc(tfc, chr, down),
                _ => Self::simulate_key_by_tfc(tfc, key, down),
            }
        } else {
            anyhow::bail!("Not found TFC Context")
        }
    }
}

impl Default for Enigo {
    fn default() -> Self {
        let is_x11 = "x11" == hbb_common::platform::linux::get_display_server();
        Self {
            is_x11,
            tfc: if is_x11 {
                match TFC_Context::new() {
                    Ok(ctx) => Some(ctx),
                    Err(..) => {
                        println!("kbd context error");
                        None
                    }
                }
            } else {
                None
            },
            custom_keyboard: None,
            custom_mouse: None,
            xdo: EnigoXdo::default(),
            keyboarder: {
                let conn = Connection::init()
                    .map_err(|err| log::error!("Failed to init XConnection: {:?}", err))
                    .ok();
                conn.map(|conn| Simulator::new(&conn))
            },
        }
    }
}

impl MouseControllable for Enigo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn mouse_move_to(&mut self, x: i32, y: i32) {
        if self.is_x11 {
            self.xdo.mouse_move_to(x, y);
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_move_to(x, y)
        }
    }
    fn mouse_move_relative(&mut self, x: i32, y: i32) {
        if self.is_x11 {
            self.xdo.mouse_move_relative(x, y);
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_move_relative(x, y)
        }
    }
    fn mouse_down(&mut self, button: MouseButton) -> crate::ResultType {
        if self.is_x11 {
            self.xdo.mouse_down(button)
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_down(button)
        } else {
            Ok(())
        }
    }
    fn mouse_up(&mut self, button: MouseButton) {
        if self.is_x11 {
            self.xdo.mouse_up(button)
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_up(button)
        }
    }
    fn mouse_click(&mut self, button: MouseButton) {
        if self.is_x11 {
            self.xdo.mouse_click(button)
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_click(button)
        }
    }
    fn mouse_scroll_x(&mut self, length: i32) {
        if self.is_x11 {
            self.xdo.mouse_scroll_x(length)
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_scroll_x(length)
        }
    }
    fn mouse_scroll_y(&mut self, length: i32) {
        if self.is_x11 {
            self.xdo.mouse_scroll_y(length)
        } else if let Some(mouse) = &mut self.custom_mouse {
            mouse.mouse_scroll_y(length)
        }
    }
}

fn get_led_state(key: Key) -> bool {
    let led_file = match key {
        // FIXME: the file may be /sys/class/leds/input2 or input5 ...
        Key::CapsLock => "/sys/class/leds/input1::capslock/brightness",
        Key::NumLock => "/sys/class/leds/input1::numlock/brightness",
        _ => {
            return false;
        }
    };

    let status = if let Ok(mut file) = std::fs::File::open(&led_file) {
        let mut content = String::new();
        file.read_to_string(&mut content).ok();
        let status = content.trim_end().to_string().parse::<i32>().unwrap_or(0);
        status
    } else {
        0
    };
    status == 1
}

impl KeyboardControllable for Enigo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_key_state(&mut self, key: Key) -> bool {
        if self.is_x11 {
            self.xdo.get_key_state(key)
        } else if let Some(keyboard) = &mut self.custom_keyboard {
            keyboard.get_key_state(key)
        } else {
            get_led_state(key)
        }
    }

    fn key_sequence(&mut self, sequence: &str) {
        if self.is_x11 {
            self.xdo.key_sequence(sequence)
        } else if let Some(keyboard) = &mut self.custom_keyboard {
            keyboard.key_sequence(sequence)
        }
    }

    fn key_down(&mut self, key: Key) -> crate::ResultType {
        if self.is_x11 {
            if let Err(err) = self.simulate_by_tfc(key, true) {
                log::warn!("Failed to simulate key: {:?} => up", err);
                self.xdo.key_down(key)
            } else {
                Ok(())
            }
        } else if let Some(keyboard) = &mut self.custom_keyboard {
            keyboard.key_down(key)
        } else {
            Ok(())
        }
    }
    fn key_up(&mut self, key: Key) {
        if self.is_x11 {
            if let Err(err) = self.simulate_by_tfc(key, false) {
                log::warn!("Failed to simulate key: {:?} => down", err);
                self.xdo.key_up(key);
            }
        } else if let Some(keyboard) = &mut self.custom_keyboard {
            keyboard.key_up(key)
        } else {
            log::error!("Not found uinput keyboard");
        }
    }
    fn key_click(&mut self, key: Key) {
        self.key_down(key).ok();
        self.key_up(key);
    }
}

fn convert_to_tfc_key(key: Key) -> Option<TFC_Key> {
    let key = match key {
        Key::Alt => TFC_Key::Alt,
        Key::Backspace => TFC_Key::DeleteOrBackspace,
        Key::CapsLock => TFC_Key::CapsLock,
        Key::Control => TFC_Key::Control,
        Key::Delete => TFC_Key::ForwardDelete,
        Key::DownArrow => TFC_Key::DownArrow,
        Key::End => TFC_Key::End,
        Key::Escape => TFC_Key::Escape,
        Key::F1 => TFC_Key::F1,
        Key::F10 => TFC_Key::F10,
        Key::F11 => TFC_Key::F11,
        Key::F12 => TFC_Key::F12,
        Key::F2 => TFC_Key::F2,
        Key::F3 => TFC_Key::F3,
        Key::F4 => TFC_Key::F4,
        Key::F5 => TFC_Key::F5,
        Key::F6 => TFC_Key::F6,
        Key::F7 => TFC_Key::F7,
        Key::F8 => TFC_Key::F8,
        Key::F9 => TFC_Key::F9,
        Key::Home => TFC_Key::Home,
        Key::LeftArrow => TFC_Key::LeftArrow,
        Key::PageDown => TFC_Key::PageDown,
        Key::PageUp => TFC_Key::PageUp,
        Key::Return => TFC_Key::ReturnOrEnter,
        Key::RightArrow => TFC_Key::RightArrow,
        Key::Shift => TFC_Key::Shift,
        Key::Space => TFC_Key::Space,
        Key::Tab => TFC_Key::Tab,
        Key::UpArrow => TFC_Key::UpArrow,
        Key::Numpad0 => TFC_Key::N0,
        Key::Numpad1 => TFC_Key::N1,
        Key::Numpad2 => TFC_Key::N2,
        Key::Numpad3 => TFC_Key::N3,
        Key::Numpad4 => TFC_Key::N4,
        Key::Numpad5 => TFC_Key::N5,
        Key::Numpad6 => TFC_Key::N6,
        Key::Numpad7 => TFC_Key::N7,
        Key::Numpad8 => TFC_Key::N8,
        Key::Numpad9 => TFC_Key::N9,
        Key::Decimal => TFC_Key::NumpadDecimal,
        Key::Clear => TFC_Key::NumpadClear,
        Key::Pause => TFC_Key::PlayPause,
        Key::Print => TFC_Key::Print,
        Key::Snapshot => TFC_Key::PrintScreen,
        Key::Insert => TFC_Key::Insert,
        Key::Scroll => TFC_Key::ScrollLock,
        Key::NumLock => TFC_Key::NumLock,
        Key::RWin => TFC_Key::Meta,
        Key::Apps => TFC_Key::Apps,
        Key::Multiply => TFC_Key::NumpadMultiply,
        Key::Add => TFC_Key::NumpadPlus,
        Key::Subtract => TFC_Key::NumpadMinus,
        Key::Divide => TFC_Key::NumpadDivide,
        Key::Equals => TFC_Key::NumpadEquals,
        Key::NumpadEnter => TFC_Key::NumpadEnter,
        Key::RightShift => TFC_Key::RightShift,
        Key::RightControl => TFC_Key::RightControl,
        Key::RightAlt => TFC_Key::RightAlt,
        Key::Command | Key::Super | Key::Windows | Key::Meta => TFC_Key::Meta,
        _ => {
            return None;
        }
    };
    Some(key)
}

#[cfg(test)]
mod test {
    use crate::{Enigo, Key, KeyboardControllable};
    use hbb_common::{anyhow, env_logger};

    #[test]
    fn test_simulate_char() -> anyhow::Result<()> {
        env_logger::init_from_env(
            env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
        );
        std::env::set_var("DISPLAY", ":0");

        let mut enigo: Enigo = Default::default();
        // normal char
        enigo.key_down(Key::Layout('1'))?;
        enigo.key_up(Key::Layout('1'));
        // dead char
        enigo.key_down(Key::Layout('â'))?;
        enigo.key_up(Key::Layout('â'));

        Ok(())
    }

    #[test]
    fn test_get_key_state() {
        let mut enigo = Enigo::new();

        for k in [Key::CapsLock, Key::NumLock] {
            enigo.key_click(k);
            let a = enigo.get_key_state(k);
            enigo.key_click(k);
            let b = enigo.get_key_state(k);
            assert!(a != b);
        }

        for k in [Key::Control, Key::Alt, Key::Shift] {
            enigo.key_down(k).ok();
            let a = enigo.get_key_state(k);
            enigo.key_up(k);
            let b = enigo.get_key_state(k);
            assert!(a != b);
        }
    }
}
