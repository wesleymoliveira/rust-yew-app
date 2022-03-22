use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::app::AppRoute, types::ArticleInfo};

const FAVORITED_CLASS: &'static str = "btn btn-sm btn-primary";
const NOT_FAVORITED_CLASS: &'static str = "btn btn-sm btn-outline-primary";

pub struct ArticlePreview {
    props: Props,
}

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub article: ArticleInfo,
}

impl Component for ArticlePreview {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ArticlePreview { props: {} }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src="{article.author.image}" />
                    <div class="info">
                    <Link <AppRoute> classes="author" to={ AppRoute::Profile  {username: ctx.props().article.author.username} }  >{  &ctx.props().article.author.username  }</Link<AppRoute>>
                        <span class="date">
                            { &ctx.props().article.created_at }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class={ NOT_FAVORITED_CLASS }>
                            <i class="ion-heart"></i> { "{article.favoritesCount}" }
                        </button>
                    </div>
                </div>



                <h1><Link<AppRoute> to={ AppRoute::Article {slug: ctx.props().article.slug.to_string()} } classes="preview-link" >{  &ctx.props().article.title  }</Link<AppRoute>></h1>
                <p>{ &ctx.props().article.description }</p>
                <span><Link<AppRoute> to={ AppRoute::Article {slug: ctx.props().article.slug.to_string()} } >{  "Read more..." }</Link<AppRoute>></span>
                <ul class="tag-list">
                    <li class="tag-default tag-pill tag-outline" key="{tag}">
                        { "{tag}" }
                    </li>
                </ul>
            </div>
        }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
