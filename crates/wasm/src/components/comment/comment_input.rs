use yew::prelude::*;

pub struct CommentInput {}

pub enum Msg {}

impl Component for CommentInput {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CommentInput {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "CommentInput" }
            </>
        }
    }
}
