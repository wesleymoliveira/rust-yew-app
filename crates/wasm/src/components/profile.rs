use yew::prelude::*;

pub struct Profile {}

pub enum Msg {}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Profile {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { "Profile" }
            </>
        }
    }
}
