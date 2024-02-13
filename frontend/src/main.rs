use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "wasteof.time" }</h1>
            <p>{ "wasteof.money, but it's self-hostable!" }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}