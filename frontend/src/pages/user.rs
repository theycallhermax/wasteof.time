use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub username: String
}

#[function_component]
pub fn User(props: &Props) -> Html {
    let username = &props.username;

    html! {
        <>
            <div class="container">
                <h1>{ "@" }{ username }</h1>
                <p>{ "i use wasteof.time btw" }</p>
                <button>{ "Wall" }</button>

                <br />

                <div class="container border">
                    <h3>{ "@" }{ username }</h3>
                    <p>{ "Lorem ispum dolor sit amet" }</p>
                </div>
                <div class="container border">
                    <h3>{ "@" }{ username }</h3>
                    <p>{ "imagine existing" }</p>
                </div>
            </div>
        </>
    }
}
