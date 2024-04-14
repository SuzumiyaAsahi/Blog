use crate::components::app::Route;
use crate::fetch::fetch;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::use_navigator;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    /// 设置网页的标题
    pub set_title: Callback<String>,
    /// 用户信息（是一个 State，因为我们可能修改里面的数据，并且修改后要更新
    /// 显示的数据）
    pub user: UseStateHandle<Result<User, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let set_title = Callback::from(move |content: String| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("{content} - BLOG"));
    });

    let user = use_state(|| Err("".into()));

    {
        let user = user.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                user.set(fetch::<User>("http://127.0.0.1:12345/user/info".into()).await)
            })
        });
    }

    let jump = move |route: Route| Callback::from(move |_: MouseEvent| navigator.push(&route));
    html! {
        <>
            <nav>
                <a onclick={jump(Route::Home)} class="brand">
                    <span>{ "Blog" }</span>
                </a>
            </nav>

            <ContextProvider<Callback<String>> context={set_title} >
                { for props.children.iter() }
            </ContextProvider<Callback<String>>>
        </>
    }
}
