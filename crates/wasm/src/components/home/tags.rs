use yew::prelude::*;

pub struct Tags {}

pub enum Msg {}

impl Component for Tags {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Tags {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div className="tag-list">
                 <a
                    href=""
                    class="tag-default tag-pill"
                    key=""
                    >
                    { "tags" }
                </a>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
