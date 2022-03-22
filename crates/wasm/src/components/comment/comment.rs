use yew::prelude::*;

pub struct Comment {}

pub enum Msg {}

impl Component for Comment {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Comment {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "Comment" }
            </>
        }
    }
}
