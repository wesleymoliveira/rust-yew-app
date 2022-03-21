use yew::prelude::*;

pub struct Register {}

pub enum Msg {}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Register {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>{ "Register" }</>
        }
    }
}
