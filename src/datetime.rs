// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::prelude::*;
use yew::InputData;
use web_sys::console;
use chrono;

pub struct DateTime {
    link: ComponentLink<Self>,
    state: State,
    onsignal: Callback<chrono::DateTime<chrono::Utc>>,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub onsignal: Callback<chrono::DateTime<chrono::Utc>>,
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
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                        console::log_1(&format!("Date changed {}", &datetime).into());
                    },
                    Err(e) => {
                        console::log_1(&format!("Error {}", &e).into());
                    },
                }
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                        console::log_1(&format!("Time changed {}", &datetime).into());
                    },
                    Err(e) => {
                        console::log_1(&format!("Error {}", &e).into());
                    },
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
    fn create_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>, chrono::format::ParseError> {
        format!("{}T{}Z", &self.state.date, &self.state.time).parse::<chrono::DateTime<chrono::Utc>>()
    }
}
