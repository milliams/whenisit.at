// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

#![recursion_limit = "512"]

mod utils;
mod datetime;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::datetime::DateTime;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Converter {
    link: ComponentLink<Self>,
    datetime: Option<chrono::DateTime<chrono::Utc>>,
}

pub enum Msg {
    DateTimeChanged(chrono::DateTime<chrono::Utc>),
}

impl Component for Converter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Converter {
            link,
            datetime: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DateTimeChanged(val) => {
                self.datetime = Some(val);
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
                <p>{&if let Some(x) = self.datetime {x.to_string()} else {"".into()} }</p>
                <DateTime onsignal=onsignal />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    utils::set_panic_hook();
    App::<Converter>::new().mount_to_body();
}
