// SPDX-FileCopyrightText: © 2020 Matt Williams <matt@milliams.com>
// SPDX-License-Identifier: MIT

use yew::{function_component, html, Properties};

use crate::utils;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    #[prop_or_default]
    pub datetime: Option<chrono::DateTime<chrono::Utc>>,
}

/// Displays a time in several time zones
#[function_component(TimeZoneDisplay)]
pub fn time_zone_display(props: &Props) -> Html {
    if let Some(dt) = props.datetime {
        html! {
            <div>
                <p>{"Reference time: "}{dt.to_string()}</p>
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
    } else {
        html! {
            <p>
                {"Please enter a date and time above"}
            </p>
        }
    }
}

fn convert_to_timezone(
    utc_time: &chrono::DateTime<chrono::Utc>,
    tz: chrono_tz::Tz,
) -> chrono::NaiveDateTime {
    utc_time.with_timezone(&tz).naive_local()
}
