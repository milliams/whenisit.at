// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::prelude::*;
use yew::InputData;
use web_sys::console;
use chrono;
use chrono::TimeZone;

pub struct DateTime {
    link: ComponentLink<Self>,
    state: State,
    onsignal: Callback<chrono::DateTime<chrono::FixedOffset>>,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub onsignal: Callback<chrono::DateTime<chrono::FixedOffset>>,
}

pub struct State {
    date: String,
    time: String,
    tz: String,
}

pub enum Msg {
    UpdateDate(String),
    UpdateTime(String),
}

impl Component for DateTime {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            date: "".into(),
            time: "".into(),
            tz: "".into(),
        };
        Self {
            link,
            state,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateDate(val) => {
                self.state.date = val;
                match self.create_datetime() {
                    Some(datetime) => {
                        self.onsignal.emit(datetime);
                        console::log_1(&format!("Date changed {}", &datetime).into());
                    },
                    None => {},
                }
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                match self.create_datetime() {
                    Some(datetime) => {
                        self.onsignal.emit(datetime);
                        console::log_1(&format!("Date changed {}", &datetime).into());
                    },
                    None => {},
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input type="date" class="date"
                    value=&self.state.date
                    oninput=self.link.callback(|e: InputData| Msg::UpdateDate(e.value))
                />
                <input type="time" class="date" step="1"
                    value=&self.state.time
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTime(e.value))
                />
            </div>
        }
    }
}

impl DateTime {
    fn create_datetime(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        let naive_dt = format!("{}T{}", &self.state.date, &self.state.time).parse::<chrono::NaiveDateTime>().ok()?;
        match chrono::Local::now().offset().from_local_datetime(&naive_dt) {
            chrono::offset::LocalResult::Single(dt) => Some(dt),
            chrono::offset::LocalResult::Ambiguous(_, _) => None,  // The user says 1:30 in the morning of a clock change day. *Which* 1:30?
            chrono::offset::LocalResult::None => None,
        }
    }
}
