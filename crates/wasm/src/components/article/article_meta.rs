use yew::prelude::*;

pub struct ArticleMeta {}

pub enum Msg {}

impl Component for ArticleMeta {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ArticleMeta {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "ArticleMeta" }
            </>
        }
    }
}
