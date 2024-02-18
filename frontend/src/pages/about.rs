use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <>
            <div class="container">
                <h1>
                    { "About " }
                    <span class="highlight">{ "wasteof.time." }</span>
                </h1>

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
