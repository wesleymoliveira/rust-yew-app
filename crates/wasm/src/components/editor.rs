use yew::prelude::*;

pub struct Editor {}

pub enum Msg {}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Editor {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "Editor" }
            </>
        }
    }
}
