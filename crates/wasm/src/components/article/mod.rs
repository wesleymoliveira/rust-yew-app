pub mod article_actions;
pub mod article_list;
pub mod article_meta;
pub mod article_preview;

use yew::prelude::*;

pub struct Article {}

pub enum Msg {}

impl Component for Article {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Article {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "Article" }
            </>
        }
    }
}
