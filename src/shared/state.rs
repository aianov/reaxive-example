use dioxus::prelude::*;
use crate::shared::router::Route;

#[derive(Clone)]
pub struct AppState {
    pub route: Signal<Route>,
}

pub fn init_app_state() {
    use_context_provider(|| AppState {
        route: Signal::new(Route::User),
    });
}

pub fn use_route() -> Signal<Route> {
    let state = use_context::<AppState>();
    state.route.clone()
} 