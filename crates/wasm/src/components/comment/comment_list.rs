use yew::prelude::*;

pub struct CommentList {}

pub enum Msg {}

impl Component for CommentList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CommentList {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "CommentList" }
            </>
        }
    }
}
