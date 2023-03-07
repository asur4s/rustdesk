use bitflags::*;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
