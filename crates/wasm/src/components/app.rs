use yew::prelude::*;
use yew_router::prelude::*;

use super::{
    article::Article, editor::Editor, header::Header, home::Home, login::Login, profile::Profile,
    profile_favorites::ProfileFavorites, register::Register, settings::Settings,
};

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
                AppRoute::Editor { slug } => html! {<Editor />},
                AppRoute::EditorCreate => html! {<Editor />},
                AppRoute::Article { slug } => html! {<Article />},
                AppRoute::Settings => html! {<Settings />},
                AppRoute::ProfileFavorites { username } => html! {<ProfileFavorites />},
                AppRoute::Profile { username } => html! {<Profile />},
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

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/")]
    Home,
    #[at("/editor/{slug}")]
    Editor { slug: String },
    #[at("/editor")]
    EditorCreate,
    #[at("/article/{slug}")]
    Article { slug: String },
    #[at("/settings")]
    Settings,
    #[at("/@{username}/favorites")]
    ProfileFavorites { username: String },
    #[at("/@{username}")]
    Profile { username: String },
}
