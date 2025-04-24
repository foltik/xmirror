use x11::xlib::{self, XFlush};
use x11::xtest;
use xmirror_event::{Event, Key, Mods, Mouse, Scroll};

#[derive(Clone, Copy)]
pub struct X11 {
    display: *mut xlib::Display,
}
unsafe impl Send for X11 {}
unsafe impl Sync for X11 {}

impl X11 {
    pub fn new() -> Self {
        unsafe {
            let display = xlib::XOpenDisplay(std::ptr::null());
            assert!(!display.is_null(), "Failed to open X11 display. Check $DISPLAY?");
            Self { display }
        }
    }

    pub fn emulate(&self, ev: Event) -> Result<(), Box<dyn std::error::Error>> {
        let key = |key: Key, b| unsafe {
            xtest::XTestFakeKeyEvent(self.display, key.to_x11(), b, 0);
        };
        let mouse = |m: Mouse, b| unsafe {
            xtest::XTestFakeButtonEvent(self.display, m.to_x11(), b, 0);
        };
        let r#move = |dx, dy| unsafe {
            xtest::XTestFakeRelativeMotionEvent(self.display, dx as i32, dy as i32, 0, 0);
        };
        let scroll = |dx: i8, dy: i8| {
            let dir = |s: Scroll| unsafe {
                xtest::XTestFakeButtonEvent(self.display, s.to_x11(), 1, 0);
                xtest::XTestFakeButtonEvent(self.display, s.to_x11(), 0, 0);
            };
            for _ in 0..dx.abs() {
                dir(if dx > 0 { Scroll::Right } else { Scroll::Left });
            }
            for _ in 0..dy.abs() {
                dir(if dy > 0 { Scroll::Up } else { Scroll::Down });
            }
        };
        let mods = |m: Mods| unsafe {
            const XKB_USE_CORE_KBD: u32 = 0x0100;
            xlib::XkbLockModifiers(self.display, XKB_USE_CORE_KBD, 0x7f, m.to_x11());
        };

        match ev {
            Event::KeyDown(k) => key(k, 1),
            Event::KeyUp(k) => key(k, 0),
            Event::Mods(m) => mods(m),
            Event::MouseDown(m) => mouse(m, 1),
            Event::MouseUp(m) => mouse(m, 0),
            Event::MouseMove { dx, dy } => r#move(dx, dy),
            Event::Scroll { dx, dy } => scroll(dx, dy),
        }
        unsafe { XFlush(self.display) };
        Ok(())
    }
}
