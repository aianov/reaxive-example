use dioxus::prelude::*;

#[component]
pub fn PostsPage() -> Element {
    rsx! {
        div { class: "posts-page",
            h1 { "Posts Page" }
            p { "Тут короче типо posts page" }

            div { class: "counter-section",
                h2 { "Shared Counter" }
            }

            div { class: "rustx-counter-section",
                p { "This counter is shared with User page using RustX" }
            }
        }
    }
}
