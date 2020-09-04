// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::{Properties, Html, ShouldRender, ComponentLink, Component, html};
use yewtil::NeqAssign;

use crate::utils;

/// Displays a time in several time zones
pub struct TimeZoneDisplay {
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TimeZoneDisplay {
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
                            if let Some(local_tz) = utils::get_local_timezone() {
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

fn convert_to_timezone(utc_time: &chrono::DateTime<chrono::Utc>, tz: chrono_tz::Tz) -> chrono::NaiveDateTime {
    utc_time.with_timezone(&tz).naive_local()
}
