use yew::prelude::*;

pub struct CommentContainer {}

pub enum Msg {}

impl Component for CommentContainer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        CommentContainer {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "CommentContainer" }
            </>
        }
    }
}
