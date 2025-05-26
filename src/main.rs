mod app;
mod pages;
mod shared;
mod stores;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    LaunchBuilder::desktop().launch(App);
}
