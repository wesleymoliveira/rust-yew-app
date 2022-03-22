use failure::Error;
use log::info;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::prelude::*;

use super::article_preview::ArticlePreview;
use crate::agent::Articles;
use crate::components::app::AppRoute;
use crate::types::{ArticleInfo, ArticleListInfo};

pub struct ArticleList {
    articles: Articles,
    article_list: Option<ArticleListInfo>,
    article_list_callback: Callback<Result<ArticleListInfo, Error>>,
    article_list_task: Option<FetchTask>,
}

pub enum Msg {
    ArticleList,
    ArticleListReady(Result<ArticleListInfo, Error>),
}

impl Component for ArticleList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ArticleList {
            articles: Articles::new(),
            article_list: None,
            article_list_callback: _ctx.link().callback(Msg::ArticleListReady),
            article_list_task: None,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let task = self.articles.all(0, self.article_list_callback.clone());
            self.article_list_task = Some(task);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ArticleList => {
                let task = self.articles.all(0, self.article_list_callback.clone());
                self.article_list_task = Some(task);
            }
            Msg::ArticleListReady(Ok(article_list)) => {
                self.article_list = Some(article_list);
            }
            Msg::ArticleListReady(Err(err)) => {
                // Can't load article list
                info!("{:?}", err);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(article_list) = &self.article_list {
            if article_list.articles.len() > 0 {
                html! {
                    <>
                        {for article_list.articles.iter().map(|article| {
                            self.article_preview(&article)
                        })}
                    </>
                }
            } else {
                html! {
                    <div class="article-preview">{ "No articles are here... yet." }</div>
                }
            }
        } else {
            html! {
                <div class="article-preview">{ "Loading..." }</div>
            }
        }
    }
}

impl ArticleList {
    fn article_preview(&self, article: &ArticleInfo) -> Html {
        html! {
            <div class="article-preview">
                <div class="article-meta">
                    <img src={ article.author.image } />
                    <div class="info">
                    <Link<AppRoute> to={ AppRoute::Profile {username: article.author.username} } classes="author" >{  &article.author.username  }</Link<AppRoute>>
                        <span class="date">
                            { &article.created_at }
                        </span>
                    </div>
                    <div class="pull-xs-right">
                        <button class="btn btn-sm btn-outline-primary">
                            <i class="ion-heart"></i> { &article.favorites_count }
                        </button>
                    </div>
                </div>
                <h1><Link<AppRoute> to={AppRoute::Article {slug: article.slug} } classes="preview-link" >{  &article.title  }</Link<AppRoute>></h1>
                <p>{ &article.description }</p>
                <span><Link<AppRoute> to={AppRoute::Article {slug: article.slug} } >{  "Read more..."  }</Link<AppRoute>></span>
                <ul class="tag-list">
                    {for article.tag_list.iter().map(|tag| {
                        html! {
                            <li class="tag-default tag-pill tag-outline" key={ tag.to_string() }>
                                { &tag }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }
}
