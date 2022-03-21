mod banner;
mod main_view;
mod tags;

use yew::prelude::*;

use banner::Banner;
use main_view::MainView;
use tags::Tags;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="home-page">
                <Banner />
                <div class="container page">
                    <div class="row">
                        <MainView />
                        <div class="col-md-3">
                            <div class="sidebar">
                                <p>{ "Popular Tags" }</p>
                                <Tags />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
