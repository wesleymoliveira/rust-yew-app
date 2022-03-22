use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::AppRoute;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-light">
                <div class="container">
                <Link <AppRoute> to={AppRoute::Home} classes="navbar-brand" >{ "WesApp" }</Link<AppRoute>>
                    <ul class="nav navbar-nav pull-xs-right">
                        <li class="nav-item">
                        <Link <AppRoute> to={AppRoute::Home} classes="nav-link" >{ "Home" }</Link<AppRoute>>
                        </li>
                        <li class="nav-item">
                            <Link <AppRoute> to={AppRoute::Login} classes="nav-link" >{ "Sign in" }</Link<AppRoute>>
                        </li>
                        <li class="nav-item">
                        <Link <AppRoute> to={AppRoute::Register} classes="nav-link" >{ "Sign up" }</Link<AppRoute>>

                        </li>
                    </ul>
                </div>
            </nav>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
