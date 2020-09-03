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
    UpdateTimeZone(String),
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
                        console::log_1(&format!("Date changed {}", &datetime).into());
                        self.onsignal.emit(datetime);
                    },
                    Err(e) => {console::log_1(&format!("Parsing failed: {}", &e).into()); return false;},
                }
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        console::log_1(&format!("Time changed {}", &datetime).into());
                        self.onsignal.emit(datetime);
                    },
                    Err(e) => {console::log_1(&format!("Parsing failed: {}", &e).into()); return false;},
                }
            }
            Msg::UpdateTimeZone(val) => {
                self.state.tz = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        console::log_1(&format!("TZ changed {}", &datetime).into());
                        self.onsignal.emit(datetime);
                    },
                    Err(e) => {console::log_1(&format!("Parsing failed: {}", &e).into()); return false;},
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
                <input type="time" class="time" step="1"
                    value=&self.state.time
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTime(e.value))
                />
                <input type="text" class="tz"
                    value=&self.state.tz
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTimeZone(e.value))
                />
            </div>
        }
    }
}

impl DateTime {
    fn create_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
        console::log_1(&format!("time '{}'", &format!("{}T{}", &self.state.date, &self.state.time)).into());
        let naive_dt = format!("{}T{}", &self.state.date, &self.state.time).parse::<chrono::NaiveDateTime>()?;

        match self.state.tz.as_str() {
            "" => {
                console::log_1(&"No timezone".into());
                match chrono::Local::now().offset().from_local_datetime(&naive_dt) {
                    chrono::offset::LocalResult::Single(dt) => Ok(dt.into()),
                    chrono::offset::LocalResult::Ambiguous(_, _) => Err("Ambiguous".into()),  // The user says 1:30 in the morning of a clock change day. *Which* 1:30?
                    chrono::offset::LocalResult::None => Err("None".into()),
                }
            },
            _ => {
                console::log_1(&format!("Timezone '{}'", &self.state.tz).into());
                match self.state.tz.parse::<chrono_tz::Tz>()?.from_local_datetime(&naive_dt) {
                    chrono::offset::LocalResult::Single(dt) => Ok(dt.with_timezone(&chrono::Utc)),
                    chrono::offset::LocalResult::Ambiguous(_, _) => Err("Ambiguous".into()),  // The user says 1:30 in the morning of a clock change day. *Which* 1:30?
                    chrono::offset::LocalResult::None => Err("None".into()),
                }
            },
        }
    }
}
