use std::cell::UnsafeCell;

use wx_rs::{self, CursorType, EventType};

thread_local!(
    static CURSORS: UnsafeCell<std::iter::Cycle<std::slice::Iter<'static, CursorType>>> = {
        let cursors = &[CursorType::Arrow, CursorType::None, CursorType::Ibeam, CursorType::Hand, CursorType::Pencil, CursorType::NoEntry, CursorType::Cross, CursorType::Size, CursorType::SizeNESW];
        UnsafeCell::new(cursors.iter().cycle())
    }
);

use std::os::raw::c_void;
extern "C" fn handle_event(event: *const c_void) {
    match wx_rs::get_event_type(event) {
        EventType::MouseLeftUp => {
            let cursor = CURSORS.with(|r| unsafe { r.get().as_mut().unwrap().next().unwrap() });
            println!("set_cursor: {:?}", cursor);
            wx_rs::set_cursor(*cursor);
        }
        _ => (),
    }
}

fn main() {
    wx_rs::init_app("Hello!", 400, 300);
    wx_rs::bind_canvas_events(handle_event);

    wx_rs::run_app();
}
