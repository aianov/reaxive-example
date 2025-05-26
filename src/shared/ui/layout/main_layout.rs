use dioxus::prelude::*;
use crate::shared::ui::navigation::NavBar;

#[component]
pub fn MainLayout(children: Element) -> Element {
    rsx! {
        div { class: "layout",
            div { class: "content",
                {children}
            }
            footer { class: "footer",
                NavBar {}
            }
        }
    }
} 