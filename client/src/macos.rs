use std::os::raw::c_void;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use core_foundation::base::{CFRelease, kCFAllocatorDefault};
use core_foundation::number::{CFBooleanRef, kCFBooleanTrue};
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
use core_foundation::string::{CFStringCreateWithCString, CFStringRef, kCFStringEncodingUTF8};
use core_graphics::base::{CGError, kCGErrorSuccess};
use core_graphics::display::CGDisplay;
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy,
    CGEventType, EventField,
};
use xmirror_event::{Event, Key, Mods, Mouse};

pub struct Macos(Receiver<Event>);

impl Macos {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        enable_cursor_control();
        CGDisplay::hide_cursor(&CGDisplay::main()).expect("hide_cursor");

        thread::spawn(move || {
            let tap = create_event_tap(tx); // prevent this from being drop()ed

            let source = tap.mach_port.create_runloop_source(0).expect("create_runloop_source");
            unsafe {
                CFRunLoop::get_current().add_source(&source, kCFRunLoopCommonModes);
            }

            CFRunLoop::run_current();
        });

        Self(rx)
    }

    pub fn capture(&mut self) -> Event {
        self.0.recv().unwrap()
    }
}

impl Drop for Macos {
    fn drop(&mut self) {
        // The tap goes away when the process exits, unlike the cursor visibility
        let _ = CGDisplay::show_cursor(&CGDisplay::main());
    }
}

fn create_event_tap<'a>(event_tx: Sender<Event>) -> CGEventTap<'a> {
    let callback = move |_proxy: CGEventTapProxy, ty: CGEventType, ev: &CGEvent| {
        let event = |ty: CGEventType| match ty {
            CGEventType::KeyDown => Some(Event::KeyDown(Key::from_macos(
                ev.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u32,
            )?)),
            CGEventType::KeyUp => Some(Event::KeyUp(Key::from_macos(
                ev.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u32,
            )?)),
            CGEventType::FlagsChanged => Some(Event::Mods(Mods::from_macos(ev.get_flags().bits() as u32))),
            CGEventType::MouseMoved
            | CGEventType::LeftMouseDragged
            | CGEventType::RightMouseDragged
            | CGEventType::OtherMouseDragged => Some(Event::MouseMove {
                dx: ev.get_double_value_field(EventField::MOUSE_EVENT_DELTA_X).round() as i8,
                dy: ev.get_double_value_field(EventField::MOUSE_EVENT_DELTA_Y).round() as i8,
            }),
            CGEventType::LeftMouseDown => Some(Event::MouseDown(Mouse::Left)),
            CGEventType::LeftMouseUp => Some(Event::MouseUp(Mouse::Left)),
            CGEventType::RightMouseDown => Some(Event::MouseDown(Mouse::Right)),
            CGEventType::RightMouseUp => Some(Event::MouseUp(Mouse::Right)),
            CGEventType::OtherMouseDown => Some(Event::MouseDown(Mouse::Middle)),
            CGEventType::OtherMouseUp => Some(Event::MouseUp(Mouse::Middle)),
            CGEventType::ScrollWheel => Some(Event::Scroll {
                dx: ev.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_2) as i8,
                dy: ev.get_integer_value_field(EventField::SCROLL_WHEEL_EVENT_POINT_DELTA_AXIS_1) as i8,
            }),
            _ => None,
        };

        if let Some(event) = event(ty) {
            event_tx.send(event).unwrap();
        }

        // Drop the event so it does nothing locally
        ev.set_type(CGEventType::Null);

        Some(ev.to_owned())
    };

    CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        // Events to subscribe to
        vec![
            CGEventType::KeyDown,
            CGEventType::KeyUp,
            CGEventType::FlagsChanged,
            CGEventType::LeftMouseDown,
            CGEventType::LeftMouseUp,
            CGEventType::RightMouseDown,
            CGEventType::RightMouseUp,
            CGEventType::OtherMouseDown,
            CGEventType::OtherMouseUp,
            CGEventType::MouseMoved,
            CGEventType::LeftMouseDragged,
            CGEventType::RightMouseDragged,
            CGEventType::OtherMouseDragged,
            CGEventType::ScrollWheel,
        ],
        callback,
    )
    .expect("CGEventTap")
}

fn enable_cursor_control() {
    type CGSConnectionID = u32;
    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        fn CGSSetConnectionProperty(
            cid: CGSConnectionID,
            targetCID: CGSConnectionID,
            key: CFStringRef,
            value: CFBooleanRef,
        ) -> CGError;
        fn _CGSDefaultConnection() -> CGSConnectionID;
    }

    unsafe {
        let key = CFStringCreateWithCString(
            kCFAllocatorDefault,
            c"SetsCursorInBackground".as_ptr(),
            kCFStringEncodingUTF8,
        );

        let code =
            CGSSetConnectionProperty(_CGSDefaultConnection(), _CGSDefaultConnection(), key, kCFBooleanTrue);
        if code != kCGErrorSuccess {
            panic!("CGSSetConnectionProperty: {code}");
        }

        CFRelease(key as *const c_void);
    }
}
