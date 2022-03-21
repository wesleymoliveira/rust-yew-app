use crate::components::{header::Header, home::Home, login::Login, register::Register};
use yew::prelude::*;
use yew_router::prelude::*;

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
        fn switch(routes: &AppRoute) -> Html {
            match routes {
                AppRoute::Login => html! {<Login />},
                AppRoute::Register => html! {<Register />},
                AppRoute::Home => html! {<Home />},
            }
        }

        html! {
            <>
                <Header />
                <BrowserRouter>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </BrowserRouter>

            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
}
