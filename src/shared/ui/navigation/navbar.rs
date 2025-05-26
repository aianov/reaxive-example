use crate::shared::fullscreen::toggle_fullscreen;
use crate::shared::router::Route;
use crate::shared::state::use_route;
use dioxus::prelude::*;

pub fn get_route_signal() -> Signal<Route> {
    use_signal(|| Route::User)
}

#[component]
pub fn NavBar() -> Element {
    let mut route = use_route();

    rsx! {
        nav { class: "navbar",
            div { class: "navbar-container",
                div {
                    class: "nav-link",
                    onclick: move |_| route.set(Route::User),
                    div {
                        class: if *route.read() == Route::User { "nav-item active" } else { "nav-item" },
                        "User"
                    }
                }
                div {
                    class: "nav-link",
                    onclick: move |_| route.set(Route::Posts),
                    div {
                        class: if *route.read() == Route::Posts { "nav-item active" } else { "nav-item" },
                        "Posts"
                    }
                }
                div {
                    class: "nav-link fullscreen-btn",
                    onclick: move |_| toggle_fullscreen(),
                    div {
                        class: "nav-item",
                        "⛶"
                    }
                }
            }
        }
    }
}
