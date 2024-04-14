mod components;
use components::app::App;
mod fetch;
mod models;

fn main() {
    yew::Renderer::<App>::new().render();
}
