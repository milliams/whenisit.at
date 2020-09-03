use yew::prelude::*;
use yew::InputData;
use web_sys::console;
use chrono::{DateTime, Utc, Local};

pub struct Model {
    link: ComponentLink<Self>,
    state: State,
    onsignal: Callback<()>,
}

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub onsignal: Callback<()>,
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

impl Component for Model {
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
        let local: DateTime<Local> = Local::now();
        //self.value = local.to_string();
        match msg {
            Msg::UpdateDate(val) => {
                self.state.date = val;
                self.onsignal.emit(());
            }
            Msg::UpdateTime(val) => {
                self.state.time = val;
                self.onsignal.emit(());
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
                <input type="time" class="date"
                    value=&self.state.time
                    oninput=self.link.callback(|e: InputData| Msg::UpdateTime(e.value))
                />
            </div>
        }
    }
}
