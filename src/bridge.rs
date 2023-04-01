use std::ffi::CString;
use std::os::raw::c_void;

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

mod notsafe {
    use std::os::raw::{c_char, c_int, c_void};

    use bitflags::bitflags;

    #[derive(Debug)]
    #[repr(C)]
    pub struct Size {
        pub width: u32,
        pub height: u32,
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct Point {
        pub x: c_int,
        pub y: c_int,
    }

    #[repr(C)]
    #[allow(dead_code)]
    pub struct WindowsHandle {
        pub hwnd: *mut c_void,
        pub hinstance: *mut c_void,
    }

    #[repr(C)]
    #[allow(dead_code)]
    pub struct OSXHandle {
        pub ns_window: *mut c_void,
        pub ns_view: *mut c_void,
    }

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum EventType {
        Unknown,
        MouseMotion,
        MouseLeftDown,
        MouseLeftUp,
        MouseMiddleDown,
        MouseMiddleUp,
        MouseRightDown,
        MouseRightUp,
        MouseAux1Down,
        MouseAux1Up,
        MouseAux2Down,
        MouseAux2Up,
        MouseRightDclick,
        MouseLeftDclick,
        MouseMiddleDclick,
        MouseAux1Dclick,
        MouseAux2Dclick,
        MouseWheel,
        MouseEnterWindow,
        MouseLeaveWindow,
        KeyDown,
        KeyUp,
        Resize,
        WindowMove,
        Focus,
        Timer,
        Exit,
        Menu,
    }

    bitflags! {
        pub struct Modifiers: u32 {
            const NONE    = 0b0000;
            const ALT     = 0b0001;
            const CONTROL = 0b0010;
            const SHIFT   = 0b0100;
            const META    = 0b1000;
        }
    }

    impl Modifiers {
        pub fn new(bits: u32) -> Self {
            Self { bits }
        }
    }

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CursorType {
        None,
        Arrow,
        Cross,
        Hand,
        Ibeam,
        Magnify,
        NoEntry,
        Pencil,
        Size,
        SizeNESW,
        SizeNS,
        SizeNWSE,
        SizeWE,
    }

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum WheelAxis {
        Vertical,
        Horizontal,
    }

    #[link(name = "wxbridge")]
    extern "C" {
        pub fn init_app(name: *const c_char, width: u32, height: u32);
        pub fn set_render(render: extern "C" fn());
        pub fn run_app();
        pub fn close_app();
        pub fn refresh();
        pub fn get_client_size() -> Size;
        pub fn get_display_size() -> Size;
        pub fn get_scale_factor() -> f32;
        #[allow(dead_code)]
        pub fn get_windows_raw_window_handle() -> WindowsHandle;
        #[allow(dead_code)]
        pub fn get_osx_raw_window_handle() -> OSXHandle;
        // Events
        pub fn bind_canvas_events(handle_event: extern "C" fn(event: *const c_void));
        pub fn get_event_type(event: *const c_void) -> EventType;
        pub fn get_event_key(key_event: *const c_void) -> c_int;
        pub fn get_event_char(key_event: *const c_void) -> u32;
        pub fn get_modifiers(key_event: *const c_void) -> u32;
        pub fn shift_down(key_event: *const c_void) -> bool;
        pub fn get_event_focused(focus_event: *const c_void) -> bool;
        pub fn get_event_id(menu_event: *const c_void) -> i32;
        pub fn get_mouse_position(mouse_event: *const c_void) -> Point;
        pub fn get_mouse_wheel_rotation(mouse_event: *const c_void) -> c_int;
        pub fn get_mouse_wheel_delta(mouse_event: *const c_void) -> c_int;
        pub fn get_mouse_wheel_axis(mouse_event: *const c_void) -> WheelAxis;
        // Cursor
        pub fn set_cursor(cursor: CursorType);
        // Status Bar
        pub fn set_status_text(cursor: *const c_char);
        pub fn create_status_bar();
        // Clipboard
        pub fn put_string_on_clipboard(string: *const c_char);
        pub fn put_buffer_on_clipboard(buf: *const u8, len: u32);
        pub fn get_string_from_clipboard(string: *mut u8);
        pub fn get_clipboard_string_len() -> i32;
        pub fn get_buffer_from_clipboard(buff: *mut u8);
        pub fn get_clipboard_buffer_len() -> i32;
        // Menus
        pub fn create_menu() -> *const c_void;
        pub fn insert_to_menu(
            menu: *const c_void,
            i: u32,
            entry: *const c_char,
            help: *const c_char,
        ) -> i32;
        pub fn insert_submenu(
            menu: *const c_void,
            i: u32,
            submenu: *const c_void,
            entry: *const c_char,
            help: *const c_char,
        );
        pub fn insert_separator_to_menu(menu: *const c_void, i: u32);
        pub fn remove_from_menu(menu: *const c_void, i: u32);
        pub fn enable_menu_item(menu: *const c_void, i: u32, enable: bool);
        pub fn set_status_menu(menu: *const c_void);
        pub fn delete_menu(menu: *const c_void);
        pub fn create_menu_bar() -> *const c_void;
        pub fn insert_to_menu_bar(
            menu_bar: *const c_void,
            menu: *const c_void,
            i: u32,
            entry: *const c_char,
        );
        pub fn remove_from_menu_bar(menu_bar: *const c_void, i: u32);
        pub fn set_menu_bar(menu_bar: *const c_void);
        pub fn delete_menu_bar(menu_bar: *const c_void);
    }
}

pub use notsafe::CursorType;
pub use notsafe::EventType;
pub use notsafe::Modifiers;
pub use notsafe::Point;
pub use notsafe::Size;
pub use notsafe::WheelAxis;

pub enum Data {
    String(String),
    Custom(Vec<u8>),
}

pub fn init_app(name: &str, width: u32, height: u32) {
    unsafe {
        let s = CString::new(name).unwrap();
        notsafe::init_app(s.as_ptr(), width, height);
    };
}

pub fn run_app() {
    unsafe {
        notsafe::run_app();
    };
}

pub fn close_app() {
    unsafe {
        notsafe::close_app();
    };
}

pub fn refresh() {
    unsafe {
        notsafe::refresh();
    };
}

pub fn set_render(render: extern "C" fn()) {
    unsafe {
        notsafe::set_render(render);
    };
}

pub fn get_client_size() -> Size {
    unsafe { notsafe::get_client_size() }
}

pub fn get_display_size() -> Size {
    unsafe { notsafe::get_display_size() }
}

pub fn get_scale_factor() -> f32 {
    unsafe { notsafe::get_scale_factor() }
}

pub fn bind_canvas_events(handle_event: extern "C" fn(event: *const c_void)) {
    unsafe { notsafe::bind_canvas_events(handle_event) };
}

// Clipboard
pub fn put_on_clipboard(data: &Data) {
    match data {
        Data::String(s) => unsafe {
            let s = CString::new(s.as_str()).unwrap();
            notsafe::put_string_on_clipboard(s.as_ptr())
        },
        Data::Custom(b) => unsafe { notsafe::put_buffer_on_clipboard(b.as_ptr(), b.len() as u32) },
    }
}

pub fn get_from_clipboard() -> Option<Data> {
    // Try getting a Data::Custom
    let len = unsafe { notsafe::get_clipboard_buffer_len() };
    if len > 0 {
        let mut dest: Vec<u8> = vec![0; len as usize];
        unsafe {
            notsafe::get_buffer_from_clipboard(dest.as_mut_ptr());
        }
        return Some(Data::Custom(dest));
    }

    // Try getting a Data::String
    let len = unsafe { notsafe::get_clipboard_string_len() };
    if len > 0 {
        let mut dest: Vec<u8> = vec![0; len as usize];
        unsafe {
            notsafe::get_string_from_clipboard(dest.as_mut_ptr());
        }
        if let Ok(str) = String::from_utf8(dest) {
            Some(Data::String(str))
        } else {
            None
        }
    } else {
        None
    }
}

// Events
pub fn get_event_type(event: *const c_void) -> EventType {
    unsafe { notsafe::get_event_type(event) }
}

pub fn get_event_key(key_event: *const c_void) -> i32 {
    unsafe { notsafe::get_event_key(key_event) }
}

pub fn get_modifiers(key_event: *const c_void) -> Modifiers {
    unsafe { Modifiers::new(notsafe::get_modifiers(key_event)) }
}

pub fn get_event_string(key_event: *const c_void) -> Option<String> {
    unsafe {
        let c = notsafe::get_event_char(key_event);
        if c == 0 || c == 8 || c == 9 || c == 13 || c == 27 || c == 127 {
            // Ignore control keys, tab, backspace, enter, escape
            None
        } else if get_modifiers(key_event) - Modifiers::SHIFT != Modifiers::NONE {
            // If there are non-shift modifiers, don't create a text event
            None
        } else {
            if let Some(ch) = std::char::from_u32(c) {
                if shift_down(key_event) {
                    Some(match c {
                        // Uppercase special characters are not returned by wxWidgets
                        // <lowercase_special_key_char> => <String>
                        44 => "<".to_string(),
                        45 => "_".to_string(),
                        46 => ">".to_string(),
                        47 => "?".to_string(),
                        48 => ")".to_string(),
                        49 => "!".to_string(),
                        50 => "@".to_string(),
                        51 => "#".to_string(),
                        52 => "$".to_string(),
                        53 => "%".to_string(),
                        54 => "^".to_string(),
                        55 => "&".to_string(),
                        56 => "*".to_string(),
                        57 => "(".to_string(),
                        59 => ":".to_string(),
                        61 => "+".to_string(),
                        91 => "{".to_string(),
                        92 => "|".to_string(),
                        93 => "}".to_string(),
                        96 => "~".to_string(),
                        _ => ch.to_string(),
                    })
                } else {
                    // Lowercase text characters are not returned by wxWidgets
                    Some(ch.to_lowercase().to_string())
                }
            } else {
                None
            }
        }
    }
}

pub fn shift_down(key_event: *const c_void) -> bool {
    unsafe { notsafe::shift_down(key_event) }
}

pub fn get_event_focused(focus_event: *const c_void) -> bool {
    unsafe { notsafe::get_event_focused(focus_event) }
}

pub fn get_event_id(menu_event: *const c_void) -> i32 {
    unsafe { notsafe::get_event_id(menu_event) }
}

pub fn get_mouse_position(mouse_event: *const c_void) -> Point {
    unsafe { notsafe::get_mouse_position(mouse_event) }
}

pub fn get_mouse_wheel_rotation(mouse_event: *const c_void) -> i32 {
    unsafe { notsafe::get_mouse_wheel_rotation(mouse_event) }
}

pub fn get_mouse_wheel_delta(mouse_event: *const c_void) -> i32 {
    unsafe { notsafe::get_mouse_wheel_delta(mouse_event) }
}

pub fn get_mouse_wheel_axis(mouse_event: *const c_void) -> WheelAxis {
    unsafe { notsafe::get_mouse_wheel_axis(mouse_event) }
}

// Cursor
pub fn set_cursor(cursor: CursorType) {
    unsafe { notsafe::set_cursor(cursor) }
}

// Status bar
pub fn set_status_text(text: &str) {
    let s = CString::new(text).unwrap();
    unsafe { notsafe::set_status_text(s.as_ptr()) }
}

pub fn create_status_bar() {
    unsafe { notsafe::create_status_bar() }
}

// Menus
pub(crate) fn create_menu() -> *const c_void {
    unsafe { notsafe::create_menu() }
}

pub(crate) fn insert_separator_to_menu(menu: *const c_void, i: usize) {
    unsafe { notsafe::insert_separator_to_menu(menu, i as u32) }
}

pub(crate) fn insert_to_menu(
    menu: *const c_void,
    i: usize,
    entry: &str,
    help: Option<&str>,
) -> i32 {
    unsafe {
        let e = CString::new(entry).unwrap();
        let h = help.map(|h| CString::new(h).unwrap());
        notsafe::insert_to_menu(
            menu,
            i as u32,
            e.as_ptr(),
            h.map_or(std::ptr::null(), |h| h.as_ptr()),
        )
    }
}

pub(crate) fn insert_submenu(
    menu: *const c_void,
    i: usize,
    submenu: *const c_void,
    entry: &str,
    help: Option<&str>,
) {
    unsafe {
        let e = CString::new(entry).unwrap();
        let h = help.map(|h| CString::new(h).unwrap());
        notsafe::insert_submenu(
            menu,
            i as u32,
            submenu,
            e.as_ptr(),
            h.map_or(std::ptr::null(), |h| h.as_ptr()),
        )
    }
}

pub(crate) fn remove_from_menu(menu: *const c_void, i: usize) {
    unsafe { notsafe::remove_from_menu(menu, i as u32) }
}

pub(crate) fn enable_menu_item(menu: *const c_void, i: usize, enable: bool) {
    unsafe { notsafe::enable_menu_item(menu, i as u32, enable) }
}

pub(crate) fn set_status_menu(menu: *const c_void) {
    unsafe { notsafe::set_status_menu(menu) }
}

pub fn delete_menu(menu: *const c_void) {
    unsafe { notsafe::delete_menu(menu) }
}

pub(crate) fn create_menu_bar() -> *const c_void {
    unsafe { notsafe::create_menu_bar() }
}

pub(crate) fn insert_to_menu_bar(
    menu_bar: *const c_void,
    menu: *const c_void,
    i: usize,
    entry: &str,
) {
    unsafe {
        let s = CString::new(entry).unwrap();
        notsafe::insert_to_menu_bar(menu_bar, menu, i as u32, s.as_ptr())
    }
}

pub(crate) fn remove_from_menu_bar(menu_bar: *const c_void, i: usize) {
    unsafe { notsafe::remove_from_menu_bar(menu_bar, i as u32) }
}

pub fn set_menu_bar(menu_bar: *const c_void) {
    unsafe { notsafe::set_menu_bar(menu_bar) }
}

pub fn delete_menu_bar(menu_bar: *const c_void) {
    unsafe { notsafe::delete_menu_bar(menu_bar) }
}

// Window
pub struct Window {}

impl Window {
    pub fn new() -> Self {
        Self {}
    }
}

unsafe impl HasRawWindowHandle for Window {
    #[cfg(windows)]
    fn raw_window_handle(&self) -> RawWindowHandle {
        unsafe {
            let h = notsafe::get_windows_raw_window_handle();
            let mut handle = raw_window_handle::Win32WindowHandle::empty();
            handle.hwnd = h.hwnd as *mut _;
            handle.hinstance = h.hinstance as *mut _;
            RawWindowHandle::Win32(handle)
        }
    }

    #[cfg(target_os = "macos")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        unsafe {
            let h = notsafe::get_osx_raw_window_handle();
            let mut handle = raw_window_handle::AppKitWindowHandle::empty();
            handle.ns_window = h.ns_window as *mut _;
            handle.ns_view = h.ns_view as *mut _;

            RawWindowHandle::AppKit(handle)
        }
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    fn raw_window_handle(&self) -> RawWindowHandle {
        panic!("Not supported")
    }
}

unsafe impl HasRawDisplayHandle for Window {
    #[cfg(windows)]
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let handle = raw_window_handle::WindowsDisplayHandle::empty();
        RawDisplayHandle::Windows(handle)
    }

    #[cfg(target_os = "macos")]
    fn raw_display_handle(&self) -> RawDisplayHandle {
        let handle = raw_window_handle::AppKitDisplayHandle::empty();
        RawDisplayHandle::AppKit(handle)
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    fn raw_display_handle(&self) -> RawDisplayHandle {
        panic!("Not supported")
    }
}
