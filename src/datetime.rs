// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::{Properties, Html, ShouldRender, ComponentLink, Component, Callback, InputData, html};
use web_sys::console;
use chrono::TimeZone;
use yew_components::Select;

use crate::utils;

/// A data, time and time zone picker
pub struct DateTime {
    link: ComponentLink<Self>,
    state: State,
    onsignal: Callback<chrono::DateTime<chrono::Utc>>,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub onsignal: Callback<chrono::DateTime<chrono::Utc>>,
}

pub struct State {
    date: String,
    time: String,
    tz: Option<chrono_tz::Tz>,
}

pub enum Msg {
    UpdateDate(String),
    UpdateTime(String),
    UpdateTimeZone(chrono_tz::Tz),
}

impl Component for DateTime {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            date: "".into(),
            time: "".into(),
            tz: None,
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
                    },
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
                    },
                }
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                    },
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
                    },
                }
            }
            Msg::UpdateTimeZone(val) => {
                self.state.tz = Some(val);
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                    },
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
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
            <div class="row">
                <input type="date" class="col-md form-control form-control-lg"
                    value=&self.state.date
                    oninput=self.link.callback(|e: InputData| Msg::UpdateDate(e.value))
                />
                <input type="time" class="col-sm form-control form-control-lg" step="1"
                    value=&self.state.time
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTime(e.value))
                />
                <Select<chrono_tz::Tz> options=chrono_tz::TZ_VARIANTS.to_vec() class="col-lg form-control form-control-lg" placeholder="Timezone" on_change=self.link.callback(|e: chrono_tz::Tz| Msg::UpdateTimeZone(e)) />  // Add "Local" timezone option
            </div>
        }
    }
}

impl DateTime {
    fn create_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
        let naive_dt = [&self.state.date, "T", &self.state.time].join("").parse::<chrono::NaiveDateTime>()?;

        let tz: chrono_tz::Tz = match self.state.tz {
            Some(tz) => {
                tz
            }
            None => {
                match utils::get_local_timezone() {
                    Some(tz) => tz,
                    None => return Err("No local timezone found".into()),
                }
            }
        };

        match tz.from_local_datetime(&naive_dt) {
            chrono::offset::LocalResult::Single(dt) => Ok(dt.with_timezone(&chrono::Utc)),
            chrono::offset::LocalResult::Ambiguous(_, _) => Err("Ambiguous".into()),  // The user says 1:30 in the morning of a clock change day. *Which* 1:30?
            chrono::offset::LocalResult::None => Err("None".into()),
        }
    }
}
