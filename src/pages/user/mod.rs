use dioxus::prelude::*;
use reaxive::prelude::*;

use crate::stores::counter_store::CounterStore;

reaxive! {
    #[component]
    pub fn UserPage() -> Element {
        let store = CounterStore::new();

        rsx! {
            div { class: "user-page",
                h1 { "ReaXive" }

                div { class: "user-section",
                    h2 { "User" }
                    div { class: "user-info",
                        p { "Full Name: {store.get_full_name()}" }
                        p { "Initials: {store.get_initials()}" }
                    }

                    div { class: "user-controls",
                        input {
                            class: "user-input",
                            placeholder: "First Name",
                            value: "{store.get_user().name}",
                            oninput: {
                                let store = store.clone();
                                move |evt: Event<FormData>| store.set_name(evt.value())
                            }
                        }
                        input {
                            class: "user-input",
                            placeholder: "Last Name",
                            value: "{store.get_user().last_name}",
                            oninput: {
                                let store = store.clone();
                                move |evt: Event<FormData>| store.set_last_name(evt.value())
                            }
                        }
                        button {
                            class: "user-btn reset",
                            onclick: {
                                let store = store.clone();
                                move |_| store.reset_user()
                            },
                            "🔄 Reset User"
                        }
                    }
                }

                div { class: "counter-section",
                    h2 { "Counter" }
                    p { "Current value: {store.get_count()}" }

                    div { class: "counter-controls",
                        button {
                            class: "counter-btn increment",
                            onclick: {
                                let store = store.clone();
                                move |_| store.increment()
                            },
                            "➕ Increment"
                        }
                        button {
                            class: "counter-btn decrement",
                            onclick: {
                                let store = store.clone();
                                move |_| store.decrement()
                            },
                            "➖ Decrement"
                        }
                    }
                }

                div { class: "info-section",
                    h3 { "About this Store" }
                    p { "This store combines user data and counter in one reactive store!" }
                    ul {
                        li { "✅ Global state management" }
                        li { "✅ Reactive with reactive! macro" }
                        li { "✅ Thread-safe" }
                        li { "✅ Zero boilerplate" }
                        li { "✅ Type-safe" }
                        li { "✅ Full class access" }
                        li { "✅ Multiple data types in one store" }
                    }
                }
            }
        }
    }
}
