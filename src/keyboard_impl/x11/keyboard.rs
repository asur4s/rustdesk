use std::collections::HashMap;

use hbb_common::{anyhow, ResultType};
use rdev::Key;
use xkbcommon::xkb::{self, x11::ffi::xkb_x11_keymap_new_from_device, Keymap};

use crate::keyboard_impl::{convert_key_to_phys, KeyOps, Modifiers, RawKeyboardEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupIndex {
    N1,
    N2,
    N3,
    N4,
}

impl From<xcb::xkb::Group> for GroupIndex {
    fn from(group: xcb::xkb::Group) -> Self {
        match group {
            xcb::xkb::Group::N1 => Self::N1,
            xcb::xkb::Group::N2 => Self::N2,
            xcb::xkb::Group::N3 => Self::N3,
            xcb::xkb::Group::N4 => Self::N4,
        }
    }
}

impl From<GroupIndex> for u32 {
    fn from(group_index: GroupIndex) -> Self {
        match group_index {
            GroupIndex::N1 => 0,
            GroupIndex::N2 => 1,
            GroupIndex::N3 => 2,
            GroupIndex::N4 => 3,
        }
    }
}

impl From<u32> for GroupIndex {
    fn from(group_id: u32) -> Self {
        match group_id {
            0 => Self::N1,
            1 => Self::N2,
            2 => Self::N3,
            3 => Self::N4,
            _ => Self::N4,
        }
    }
}

#[inline]
pub fn level_to_modifiers(level: u32) -> Modifiers {
    match level {
        0 => Modifiers::NONE,
        1 => Modifiers::SHIFT,
        2 => Modifiers::ALT_GR,
        3 => Modifiers::SHIFT | Modifiers::ALT_GR,
        _ => Modifiers::NONE,
    }
}

pub fn build_keysym_event_map(
    keymap: &xkb::Keymap,
    min_keycode: u32,
    max_keycode: u32,
    layout: u32,
) -> ResultType<HashMap<u32, RawKeyboardEvent>> {
    let mut map: HashMap<u32, RawKeyboardEvent> = HashMap::new();

    // todo
    for keycode in min_keycode..=max_keycode {
        let key = Key::from_pos(keycode)?;
        let phys = convert_key_to_phys(&key)
            .ok_or(anyhow::anyhow!("Failed to get PhysKeyCode: {:?}", &key))?;
        let num_level = keymap.num_levels_for_key(keycode, layout);
        for level in (0..num_level).rev() {
            let keysyms = keymap.key_get_syms_by_level(keycode, layout, level);
            if keysyms.is_empty() {
                continue;
            }
            let keysym = keysyms[0];
            let raw_event = RawKeyboardEvent {
                phys: phys,
                press: false,
                modifiers: level_to_modifiers(level),
                sys_code: keycode,
            };
            map.insert(keysym, raw_event);
        }
    }

    Ok(map)
}

pub fn get_active_group_index(state: &xkb::State, keymap: &xkb::Keymap) -> GroupIndex {
    let layout_num = keymap.num_layouts();
    let mut group_id = 0;
    for idx in 0..layout_num {
        let res = state.layout_index_is_active(idx, xkb::STATE_LAYOUT_LOCKED);
        if res {
            group_id = idx;
        }
    }
    GroupIndex::from(group_id)
}

pub struct XKeyboard {
    pub keysym_keycode_map: HashMap<xkb::Keysym, xkb::Keycode>,
    pub keysym_event_map: HashMap<u32, RawKeyboardEvent>,
    pub unused_keycodes: Vec<xkb::Keycode>,
    pub state: xkb::State,
    pub keymap: xkb::Keymap,
    pub device_id: u8,
}

impl XKeyboard {
    pub fn new() -> ResultType<Self> {
        let (conn, _screen_num) =
            xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;
        let connection = &conn;

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let device_id = xkb::x11::get_core_keyboard_device_id(connection);

        let keymap = xkb::x11::keymap_new_from_device(
            &context,
            connection,
            device_id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );
        let state = xkb::x11::state_new_from_device(&keymap, connection, device_id);
        let mut keysym_keycode_map = HashMap::new();
        let mut unused_keycodes: Vec<xkb::Keycode> = vec![];

        let min_keycode = keymap.min_keycode();
        let max_keycode = keymap.max_keycode();

        for keycode in min_keycode..max_keycode {
            let keysym = state.key_get_one_sym(keycode);
            if keysym == 0 {
                unused_keycodes.push(keycode);
            } else {
                keysym_keycode_map.insert(keysym, keycode);
            }
        }

        let group_index = get_active_group_index(&state, &keymap);

        let keysym_event_map: HashMap<u32, RawKeyboardEvent> =
            build_keysym_event_map(&keymap, min_keycode, max_keycode, group_index.into())?;

        Ok(Self {
            keysym_keycode_map: keysym_keycode_map,
            keysym_event_map: keysym_event_map,
            unused_keycodes: unused_keycodes,
            state: state,
            keymap: keymap,
            device_id: device_id as _,
        })
    }
}

