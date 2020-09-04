// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::{Properties, Html, ShouldRender, ComponentLink, Component, html};
use yewtil::NeqAssign;

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
                        <p>{"Local time: "}{convert_to_timezone(&dt, None).to_string()}</p>
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
