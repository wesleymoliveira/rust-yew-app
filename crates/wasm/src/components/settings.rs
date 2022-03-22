use yew::prelude::*;

pub struct Settings {}

pub enum Msg {}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Settings {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "Settings" }
            </>
        }
    }
}
