use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="header">
                <h2 class="span">{ "wasteof.time" }</h2>

                <div class="end">
                    <h3>{ "explore" }</h3>
                    // <h3 class="sides">{ "join" }</h3>
                    // <h3 class="sides">{ "login" }</h3>
                </div>
            </div>

            <div class="container">
                <h1>
                    { "Get ready to self-host the " }
                    <span class="highlight">{ "next generation social media." }</span>
                </h1>
                <p>{ "wasteof.time is a open-source version of the wasteof.money social platform." }</p>

                <h2 class="top">{ "Why?" }</h2>
                <p>
                    { "As wasteof.money is currently not open-source on neither on the frontend or backend sides, wasteof.time intends to change that, with a fully open-source structure, with both the frontend and backend open-source. This project isn't meant to take away the credit that wasteof.money deserves, but rather a way to make others" }
                    <i>{ " technically " }</i>
                    { "self-host wasteof.money." }
                </p>

                <h2 class="top">{ "What?" }</h2>
                <p>{ "wasteof.time is an open-source version of wasteof.money, meaning that you can self-host wasteof.money, or at least, something like it." }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}