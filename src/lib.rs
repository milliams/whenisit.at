// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

#![recursion_limit = "512"]

mod utils;
mod datetime;

use wasm_bindgen::prelude::*;
use yew::{App, Html, ShouldRender, ComponentLink, Component, virtual_dom::VNode, html};
use yew_router::{route::Route, service::RouteService, Switch};

use crate::datetime::DateTime;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Switch)]
pub enum AppRoute {
  #[to="/{id}"]
  GivenTime(chrono::DateTime<chrono::Utc>),
  #[to="/"]
  Home,
}

pub struct Converter {
    link: ComponentLink<Self>,
    datetime: Option<chrono::DateTime<chrono::Utc>>,
    route_service: RouteService<()>,
    route: Route<()>,
}

pub enum Msg {
    DateTimeChanged(chrono::DateTime<chrono::Utc>),
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
}

impl Component for Converter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        route_service.register_callback(callback);
        Converter {
            link,
            datetime: None,
            route_service,
            route,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DateTimeChanged(val) => {
                self.datetime = Some(val);
            }
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                // This might be derived in the future
                self.route = route.into();
                self.route_service.set_route(&self.route.route, ());
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let onsignal = &self.link.callback(Msg::DateTimeChanged);
        html! {
            <div>
                <h1>{"When is it at"}</h1>
                {
                    match AppRoute::switch(self.route.clone()) {
                        Some(AppRoute::Home) => { html! {
                            <div>
                                <p>{&if let Some(x) = self.datetime {x.to_string()} else {"".into()} }</p>
                                <DateTime onsignal=onsignal />
                            </div>
                        }},
                        Some(AppRoute::GivenTime(dt)) => { html! {
                            <div>
                                <p>{"Reference time: "}{&dt.to_string()}</p>
                                <p>{"Local time: "}{convert_to_timezone(&dt, None).to_string()}</p>
                            </div>
                        }},
                        None => VNode::from("404")
                    }
                }
            </div>
        }
    }
}

fn convert_to_timezone(utc_time: &chrono::DateTime<chrono::Utc>, tz: Option<chrono_tz::Tz>) -> chrono::NaiveDateTime {
    match tz {
        None => {
            let local_dt: chrono::DateTime<chrono::Local> = (*utc_time).into();
            local_dt.naive_local()
        }
        Some(timezone) => {
            utc_time.with_timezone(&timezone).naive_local()
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    utils::set_panic_hook();
    App::<Converter>::new().mount_to_body();
}
