use yew::prelude::*;
use yew_router::prelude::*;

use crate::{components::app::Route, fetch::fetch, models::article::ArticlePreview as Preview};

#[function_component(ArticlePreview)]
pub fn article_preview() -> Html {
    let loading = use_state(|| true);
    let articles = use_state(|| Err("".into()));

    let navigator = use_navigator().unwrap();
    {
        let loading = loading.clone();
        let articles = articles.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // get 请求是不允许http请求有body的。
                articles.set(fetch::<Vec<Preview>>("http://127.0.0.1:12345/articles".into()).await);
                loading.set(false);
            });
        });
    }
    html! {
        if *loading {
            <p>{ "Loading ... " }</p>
        } else {
            { content(navigator ,(*articles).clone()) }
        }
    }
}

fn content(navigator: Navigator, articles: Result<Vec<Preview>, String>) -> Html {
    let jump = |navigator: Navigator, article_id: u32| -> Callback<MouseEvent> {
        Callback::from(move |_| navigator.push(&Route::ArticleViewer { article_id }))
    };
    match articles {
        Ok(articles) => articles
            .iter()
            .map(|i| {
                html! {
                <article class="card" onclick={jump(navigator.clone() ,i.id)} key={i.id}>
                        <header>
                            <h3>{ &i.title }</h3>
                            <span style="color: grey;"> { &i.date }</span>
                        </header>
                </article>
                }
            })
            .collect::<Html>(),
        Err(e) => html! {
            <p>{ e }</p>
        },
    }
}
