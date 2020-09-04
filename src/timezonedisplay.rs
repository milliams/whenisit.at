// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use wasm_bindgen::prelude::*;
use yew::{Properties, Html, ShouldRender, ComponentLink, Component, html};
use yewtil::NeqAssign;
use js_sys;

use web_sys::console;

pub struct TimeZoneDisplay {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
}

impl Component for TimeZoneDisplay {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TimeZoneDisplay {
            link,
            props: Props {
                datetime: props.datetime,
            }
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.props.datetime {
            Some(dt) => {
                html! {
                    <div>
                        <p>{"Reference time: "}{&dt.to_string()}</p>
                        {
                            if let Some(local_tz) = get_local_timezone() {
                                html! {
                                    <p>{local_tz.to_string()}{": "}{convert_to_timezone(&dt, local_tz).to_string()}</p>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            }
            None => {
                html! {
                    <div>
                        {"No date time to display"}
                    </div>
                }
            }
        }
    }
}

fn get_local_timezone() ->  Option<chrono_tz::Tz> {
    let options = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &js_sys::Object::new()).resolved_options();
    let tz2 = js_sys::Reflect::get(&options, &JsValue::from("timeZone")).ok()?.as_string()?;
    tz2.parse().ok()
}

fn convert_to_timezone(utc_time: &chrono::DateTime<chrono::Utc>, tz: chrono_tz::Tz) -> chrono::NaiveDateTime {
    utc_time.with_timezone(&tz).naive_local()
}
