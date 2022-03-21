use yew::prelude::*;

pub struct Banner {}

pub enum Msg {}

impl Component for Banner {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Banner {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            <div class="banner">
                <div class="container">
                    <h1 class="logo-font">
                        { "WesApp" }
                    </h1>
                    <p>{ "My first real Yew-App" }</p>
                </div>
            </div>
        </>
                        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
