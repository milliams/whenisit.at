// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

#![recursion_limit = "512"]

mod datetime;
mod timezonedisplay;
mod utils;

use wasm_bindgen::prelude::*;
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::{BrowserRouter, Routable, Switch};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/:dt")]
    GivenTime { dt: chrono::DateTime<chrono::Utc> },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {
            <Main />
        },
        AppRoute::GivenTime { dt } => html! {
            <timezonedisplay::TimeZoneDisplay datetime={*dt} />
        },
        AppRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[function_component(Main)]
fn main() -> Html {
    let datetime = use_state(|| None);
    let updatetime = {
        let datetime = datetime.clone();
        Callback::from(move |dt: chrono::DateTime<chrono::Utc>| datetime.set(Some(dt)))
    };

    html! {
        <div>
            <p class="lead">
                {"Set a time and a date and share it across time zones"}
            </p>
            <datetime::DateTime onsignal={updatetime} />
            <timezonedisplay::TimeZoneDisplay datetime={*datetime} />
            {
                if let Some(dt) = *datetime {
                    html! {<a href={dt.to_rfc3339()}>{"Share time"}</a>}
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    utils::set_panic_hook();
    let main = gloo_utils::document()
        .query_selector("main")
        .unwrap()
        .unwrap();
    yew::start_app_in_element::<App>(main);
}
