use pulldown_cmark::{html, Options, Parser};
use web_sys::Node;
use yew::prelude::*;

use crate::{components::card::Card, fetch, models::article::Article};

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub article_id: u32,
}

#[function_component(ArticleViewer)]
pub fn article_viewer(props: &Props) -> Html {
    let loading = use_state(|| true);
    let article = use_state(|| Err("".into()));

    let article_id = props.article_id;

    {
        let loading = loading.clone();
        let article = article.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                article.set(
                    fetch::fetch::<Article>(format!("http://127.0.0.1:12345/article/{article_id}"))
                        .await,
                );
                loading.set(false)
            })
        })
    }

    let title = if let Ok(article) = (*article).clone() {
        article.title
    } else {
        "文章".into()
    };

    use_context::<Callback<String>>()
        .unwrap()
        .emit(title.clone());

    html! {
        if *loading {
           <Card title={"Loading..."}>
               <p>{ "马上就好 ....." }</p>
           </Card>
        } else {
            <Card {title}>
            {
                match &*article {
                    Ok(article) => convert_markdown_to_html(article),
                    Err(e) => html! {
                        <p>{ e }</p>
                    }
                }
            }
            </Card>
        }
    }
}

fn convert_markdown_to_html(article: &Article) -> Html {
    let article_content = article.content.as_str();
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(article_content, options);

    let mut markdown_html = String::new();

    html::push_html(&mut markdown_html, parser);

    let div_wrapper = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div_wrapper.set_inner_html(&markdown_html);

    let node: Node = div_wrapper.into();

    Html::VRef(node)
}
