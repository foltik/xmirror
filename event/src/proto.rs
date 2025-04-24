//! Compact wire protocol for `Event` relying on stable ABI and enum discriminants via #[repr(C)]

use std::io::{self, Read, Write};
use std::mem::transmute;

use crate::{Event, Key, Mods, Mouse};

impl Event {
    /// Maximum size of an Event packet
    pub const MTU: usize = 3;

    #[rustfmt::skip]
    pub fn encode<W: Write>(self, w: &mut W) -> io::Result<()> {
        match self {
            Event::KeyDown(k)   => w.write_all(&[1, k as u8])?,
            Event::KeyUp(k)     => w.write_all(&[2, k as u8])?,
            Event::Mods(m)      => w.write_all(&[3, m.bits()])?,
            Event::MouseDown(b) => w.write_all(&[4, b as u8])?,
            Event::MouseUp(b)   => w.write_all(&[5, b as u8])?,
            Event::MouseMove { dx, dy } => unsafe { w.write_all(&[6, transmute(dx), transmute(dy)])? },
            Event::Scroll { dx, dy } => unsafe { w.write_all(&[7, transmute(dx), transmute(dy)])? },
        }
        Ok(())
    }

    pub fn decode<R: Read>(r: &mut R) -> io::Result<Event> {
        let tag = read_u8(r)?;
        let ev = match tag {
            1 => Event::KeyDown(read_key(read_u8(r)?)?),
            2 => Event::KeyUp(read_key(read_u8(r)?)?),
            3 => Event::Mods(Mods::from_bits_truncate(read_u8(r)?)),
            4 => Event::MouseDown(read_button(read_u8(r)?)?),
            5 => Event::MouseUp(read_button(read_u8(r)?)?),
            6 => unsafe { Event::MouseMove { dx: transmute(read_u8(r)?), dy: transmute(read_u8(r)?) } },
            7 => unsafe { Event::Scroll { dx: transmute(read_u8(r)?), dy: transmute(read_u8(r)?) } },
            _ => return Err(io::ErrorKind::InvalidData.into()),
        };
        Ok(ev)
    }
}

#[inline]
fn read_key(byte: u8) -> io::Result<Key> {
    const KEY_MAX: u8 = Key::F12 as u8 + 1;
    if byte < KEY_MAX {
        Ok(unsafe { transmute(byte) })
    } else {
        Err(io::ErrorKind::InvalidData.into())
    }
}

#[inline]
fn read_button(byte: u8) -> io::Result<Mouse> {
    const BUTTON_MAX: u8 = Mouse::Middle as u8 + 1;
    if byte < BUTTON_MAX {
        Ok(unsafe { transmute(byte) })
    } else {
        Err(io::ErrorKind::InvalidData.into())
    }
}

#[inline]
fn read_u8<R: Read>(r: &mut R) -> io::Result<u8> {
    let mut b = [0];
    r.read_exact(&mut b)?;
    Ok(b[0])
}
