use crate::components::home::Home;
use yew::prelude::*;

pub enum Msg {
    DoIt,
}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DoIt => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let _link = ctx.link();
        html! {
            <div class="container mx-1 px-1 flex-col">
                <Home />
            </div>
        }
    }
}
