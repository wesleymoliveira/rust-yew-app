use yew::prelude::*;

pub struct DeleteButton {}

pub enum Msg {}

impl Component for DeleteButton {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DeleteButton {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "DeleteButton" }
            </>
        }
    }
}
