//! Compact wire protocol for `Event` relying on stable ABI and enum discriminants via #[repr(C)]

use std::io::{self, Write};

use crate::{Event, Key, Mods, Mouse};

impl Event {
    pub const SIZE: usize = 3;

    #[rustfmt::skip]
    pub fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        match self {
            Event::KeyDown(k)   => w.write_all(&[1, k as u8,  0])?,
            Event::KeyUp(k)     => w.write_all(&[2, k as u8,  0])?,
            Event::Mods(m)      => w.write_all(&[3, m.bits(), 0])?,
            Event::MouseDown(b) => w.write_all(&[4, b as u8,  0])?,
            Event::MouseUp(b)   => w.write_all(&[5, b as u8,  0])?,
            Event::MouseMove { dx, dy } => w.write_all(&[6, dx as u8, dy as u8])?,
            Event::Scroll { dx, dy } => w.write_all(&[7, dx as u8, dy as u8])?,
        }
        Ok(())
    }

    pub fn decode(packet: &[u8; Self::SIZE]) -> io::Result<Event> {
        let [tag, byte0, byte1] = *packet;
        let bits = byte0;
        let (dx, dy) = (byte0 as i8, byte1 as i8);

        let ev = match tag {
            1 => Event::KeyDown(to_key(bits)?),
            2 => Event::KeyUp(to_key(bits)?),
            3 => Event::Mods(Mods::from_bits_truncate(bits)),
            4 => Event::MouseDown(to_button(bits)?),
            5 => Event::MouseUp(to_button(bits)?),
            6 => Event::MouseMove { dx, dy },
            7 => Event::Scroll { dx, dy },
            _ => return Err(io::ErrorKind::InvalidData.into()),
        };
        Ok(ev)
    }
}

#[inline]
fn to_key(byte: u8) -> io::Result<Key> {
    const KEY_MAX: u8 = Key::F12 as u8 + 1;
    if byte < KEY_MAX {
        Ok(unsafe { std::mem::transmute(byte) })
    } else {
        Err(io::ErrorKind::InvalidData.into())
    }
}

#[inline]
fn to_button(byte: u8) -> io::Result<Mouse> {
    const BUTTON_MAX: u8 = Mouse::Middle as u8 + 1;
    if byte < BUTTON_MAX {
        Ok(unsafe { std::mem::transmute(byte) })
    } else {
        Err(io::ErrorKind::InvalidData.into())
    }
}
