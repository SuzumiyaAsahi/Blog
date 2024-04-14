use yew::prelude::*;

use crate::components::article::article_preview::ArticlePreview;
use crate::components::card::Card;

#[function_component(Home)]
pub fn home() -> Html {
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"文章"}>
           <ArticlePreview/>
        </Card>
    }
}
