use crate::components::article::article_list::ArticleList;
use yew::prelude::*;

pub struct MainView {}

pub enum Msg {}

impl Component for MainView {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainView {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="col-md-9">
                <div class="feed-toggle">
                    <ul class="nav nav-pills outline-active">
                        <li className="nav-item">
                            <a
                                href=""
                                class="nav-link active"
                                >
                                { "Global Feed" }
                            </a>
                        </li>
                    </ul>
                </div>

                <ArticleList />
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
