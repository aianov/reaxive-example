use dioxus::prelude::*;
use crate::pages::{UserPage, PostsPage};
use crate::shared::ui::layout::MainLayout;
use crate::shared::state::{use_route, init_app_state};

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    User,
    Posts,
    NotFound,
}

#[component]
pub fn Router() -> Element {
    init_app_state();
    
    let route = use_route();
    
    rsx! {
        MainLayout {
            {match *route.read() {
                Route::User => rsx! { UserPage {} },
                Route::Posts => rsx! { PostsPage {} },
                Route::NotFound => rsx! { div { "Страница не найдена" } },
            }}
        }
    }
} 