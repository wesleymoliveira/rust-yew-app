use yew::prelude::*;

pub struct ArticleActions {}

pub enum Msg {}

impl Component for ArticleActions {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ArticleActions {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "ArticleActions" }
            </>
        }
    }
}
