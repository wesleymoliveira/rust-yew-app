use yew::prelude::*;

pub struct ListPagination {}

pub enum Msg {}

impl Component for ListPagination {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ListPagination {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "ListPagination" }
            </>
        }
    }
}
