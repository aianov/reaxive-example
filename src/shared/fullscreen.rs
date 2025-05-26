use dioxus::prelude::*;
use dioxus_desktop::use_window;

pub fn toggle_fullscreen() {
    let window = use_window();

    if window.fullscreen().is_some() {
        window.set_fullscreen(false);
    } else {
        window.set_fullscreen(true);
    }
}
