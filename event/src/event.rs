#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
    Mods(Mods),

    MouseDown(Mouse),
    MouseUp(Mouse),
    MouseMove { dx: i8, dy: i8 },
    Scroll { dx: i8, dy: i8 },
}

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Num1, Num2, Num3, Num4, Num5,
    Num6, Num7, Num8, Num9, Num0,

    Left,       Up,         Right,      Down,      Enter,
    Esc,        Backspace,  Tab,        Space,     Minus,
    Equal,      LeftBrace,  RightBrace, Backslash, SemiColon,
    Apostrophe, Grave,      Comma,      Dot,       Slash,
    CapsLock,   ScrollLock, Pause,      Print,     Insert,
    Delete,     Home,       End,        PageUp,    PageDown,

    F1, F2, F3, F4,  F5,  F6,
    F7, F8, F9, F10, F11, F12,
}

bitflags::bitflags! {
    #[rustfmt::skip]
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Mods: u8 {
        const Shift = 1 << 0;
        const Ctrl = 1 << 1;
        const Alt = 1 << 2;
        const Super = 1 << 3;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Mouse {
    Left,
    Right,
    Middle,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Scroll {
    Up,
    Down,
    Left,
    Right,
}

macro_rules! impl_conversions {
    ( $to_fn:ident, $from_fn:ident, $( $variant:ident => $id:expr ),+ $(,)? ) => {
        pub fn $to_fn(self) -> u32 {
            match self {
                $( Self::$variant => $id, )+
            }
        }
        pub fn $from_fn(id: u32) -> Option<Self> {
            match id {
                $( $id => Some(Self::$variant), )+
                _ => None,
            }
        }
    };
}
macro_rules! impl_platform {
    (
        $to_fn:ident, $from_fn:ident,
        Key { $( $key:ident => $vk:expr ),+ $(,)? }
        Mods { $( $mods:ident => $vmod:expr ),+ $(,)? }
        Mouse { $( $mouse:ident => $vm:expr ),+ $(,)? }
        Scroll { $( $scroll:ident => $vs:expr ),+ $(,)? }
    ) => {
        impl Key { impl_conversions!{ $to_fn, $from_fn, $( $key => $vk ),* } }
        impl Mouse { impl_conversions!{ $to_fn, $from_fn, $( $mouse => $vm ),* } }
        impl Scroll { impl_conversions!{ $to_fn, $from_fn, $( $scroll => $vs ),* } }
        impl Mods {
            pub fn $to_fn(self) -> u32 {
                $( (if self.contains(Self::$mods) { $vmod } else { 0 }) )|*
            }
            pub fn $from_fn(id: u32) -> Self {
                $( (if id & $vmod > 0 { Self::$mods } else { Self::empty() }) )|*
            }
        }
    }
}

impl_platform! {
    to_x11, from_x11,
    Key {
        A => 38,  B => 56,  C => 54,  D => 40,  E => 26,  F => 41,  G => 42,  H => 43,  I => 31,  J => 44,  K => 45,  L => 46,  M => 58,
        N => 57,  O => 32,  P => 33,  Q => 24,  R => 27,  S => 39,  T => 28,  U => 30,  V => 55,  W => 25,  X => 53,  Y => 29,  Z => 52,

        Num1 => 10,  Num2 => 11,  Num3 => 12,  Num4 => 13,  Num5 => 14,
        Num6 => 15,  Num7 => 16,  Num8 => 17,  Num9 => 18,  Num0 => 19,

        Left       => 113,  Up         => 111,  Right      => 114,  Down      => 116,  Enter      =>  36,
        Esc        =>   9,  Backspace  =>  22,  Tab        =>  23,  Space     =>  65,  Minus      =>  20,
        Equal      =>  21,  LeftBrace  =>  34,  RightBrace =>  35,  Backslash =>  51,  SemiColon  =>  47,
        Apostrophe =>  48,  Grave      =>  49,  Comma      =>  59,  Dot       =>  60,  Slash      =>  61,
        CapsLock   =>  66,  ScrollLock =>  78,  Pause      => 127,  Print     => 107,  Insert     => 118,
        Delete     => 119,  Home       => 110,  End        => 115,  PageUp    => 112,  PageDown   => 117,

        F1 => 67,  F2 => 68,  F3 => 69,  F4 =>  70,  F5 =>  71,  F6 =>  72,
        F7 => 73,  F8 => 74,  F9 => 75,  F10 => 76,  F11 => 95,  F12 => 96,
    }
    Mods {
        Shift => 1,  Ctrl => 4,  Alt => 8,  Super => 64,
    }
    Mouse {
        Left => 1,  Middle => 2,  Right => 3,
    }
    Scroll {
        Left => 6,  Down => 4,  Up => 5,  Right => 7,
    }
}

impl_platform! {
    to_macos, from_macos,

    Key {
        A =>  0,  B => 11,  C =>  8,  D =>  2,  E => 14,  F =>  3,  G =>  5,  H =>  4,  I => 34,  J => 38,  K => 40,  L => 37,  M => 46,
        N => 45,  O => 31,  P => 35,  Q => 12,  R => 15,  S =>  1,  T => 17,  U => 32,  V =>  9,  W => 13,  X =>  7,  Y => 16,  Z =>  6,

        Num1 => 18,  Num2 => 19,  Num3 => 20,  Num4 => 21,  Num5 => 23,
        Num6 => 22,  Num7 => 26,  Num8 => 28,  Num9 => 25,  Num0 => 29,

        Left       => 123,  Up         => 126,  Right      => 124,  Down      => 125,  Enter      =>  36,
        Esc        =>  53,  Backspace  =>  51,  Tab        =>  48,  Space     =>  49,  Minus      =>  27,
        Equal      =>  24,  LeftBrace  =>  33,  RightBrace =>  30,  Backslash =>  42,  SemiColon  =>  41,
        Apostrophe =>  39,  Grave      =>  50,  Comma      =>  43,  Dot       =>  47,  Slash      =>  44,
        CapsLock   =>  57,  ScrollLock => 107,  Pause      => 113,  Print     => 105,  Insert     => 114,
        Delete     => 117,  Home       => 115,  End        => 119,  PageUp    => 116,  PageDown   => 121,

        F1 => 122,  F2 => 120,  F3 =>  99,  F4 => 118,  F5 =>  96,  F6 =>  97,
        F7 =>  98,  F8 => 100,  F9 => 101,  F10 => 109,  F11 => 103,  F12 => 111,
    }
    Mods {
        Shift => 0x2_0000,  Ctrl => 0x4_0000,  Alt => 0x8_0000,  Super => 0x10_0000,
    }
    Mouse {
        Left => 1,  Middle => 2,  Right => 3,
    }
    Scroll {
        Left => 1,  Down => 2,  Up => 3,  Right => 4,
    }
}
