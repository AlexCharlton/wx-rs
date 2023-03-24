use std::io::{self, Write};
use wx_rs;

extern "C" fn render() {
    print!(".");
    io::stdout().flush().unwrap();
}

use std::os::raw::c_void;
extern "C" fn handle_event(event: *const c_void) {
    match wx_rs::get_event_type(event) {
        e if e != wx_rs::EventType::Timer => {
            wx_rs::set_status_text(&format!("Got event: {:?}", e));
        }
        _ => (),
    }
}

fn main() {
    println!("hello");
    wx_rs::init_app("Hello!", 400, 300);
    println!(
        "initialized window with surface size {:?}, display rect {:?} at scale {}.",
        wx_rs::get_client_size(),
        wx_rs::get_display_size(),
        wx_rs::get_scale_factor(),
    );
    wx_rs::set_render(render);
    wx_rs::bind_canvas_events(handle_event);
    wx_rs::create_status_bar();

    wx_rs::run_app();

    println!("bye");
}
