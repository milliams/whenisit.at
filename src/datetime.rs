// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use chrono::TimeZone;
use web_sys::{console, HtmlInputElement, HtmlSelectElement};
use yew::{html, Callback, Component, Context, Event, Html, Properties, TargetCast};

use crate::utils;

/// A date, time and time zone picker
pub struct DateTime {
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
    fn create(ctx: &Context<Self>) -> Self {
        let state = State {
            date: "".into(),
            time: "".into(),
            tz: None,
        };
        Self {
            state,
            onsignal: ctx.props().onsignal.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateDate(val) => {
                self.state.date = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                    }
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
                    }
                }
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                    }
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
                    }
                }
            }
            Msg::UpdateTimeZone(val) => {
                self.state.tz = Some(val);
                match self.create_datetime() {
                    Ok(datetime) => {
                        self.onsignal.emit(datetime);
                    }
                    Err(e) => {
                        console::debug_2(&"Parsing failed:".into(), &e.to_string().into());
                        return false;
                    }
                }
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                <input type="date" class="col-md form-control form-control-lg"
                    value={self.state.date.clone()}
                    onchange={ctx.link().callback(|e: Event| Msg::UpdateDate(e.target_unchecked_into::<HtmlInputElement>().value()))}
                />
                <input type="time" class="col-sm form-control form-control-lg" step="1"
                    value={self.state.time.clone()}
                    onchange={ctx.link().callback(|e: Event| Msg::UpdateTime(e.target_unchecked_into::<HtmlInputElement>().value()))}
                />
                <select class="col-lg form-control form-control-lg" placeholder="Timezone" onchange={ctx.link().callback(|e: Event| Msg::UpdateTimeZone(e.target_unchecked_into::<HtmlSelectElement>().value().parse().unwrap()))} >
                {
                    chrono_tz::TZ_VARIANTS.iter().map(|tz| {
                        html!{<option key={tz.to_string()}>{ tz }</option>}
                    }).collect::<Html>()  // TODO Add "Local" timezone option
                }
                </select>
            </div>
        }
    }
}

impl DateTime {
    fn create_datetime(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
        let naive_dt = [&self.state.date, "T", &self.state.time]
            .join("")
            .parse::<chrono::NaiveDateTime>()?;

        let tz: chrono_tz::Tz = match self.state.tz {
            Some(tz) => tz,
            None => match utils::get_local_timezone() {
                Some(tz) => tz,
                None => return Err("No local timezone found".into()),
            },
        };

        match tz.from_local_datetime(&naive_dt) {
            chrono::offset::LocalResult::Single(dt) => Ok(dt.with_timezone(&chrono::Utc)),
            chrono::offset::LocalResult::Ambiguous(_, _) => Err("Ambiguous".into()), // The user says 1:30 in the morning of a clock change day. *Which* 1:30?
            chrono::offset::LocalResult::None => Err("None".into()),
        }
    }
}
