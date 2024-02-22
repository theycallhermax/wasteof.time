use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <>
            <div class="container">
                <h1>
                    { "404 " }
                    <span class="highlight">{ "Not Found" }</span>
                </h1>
                <p>{ "You went to a page that doesn't exist..." }</p>

                <br />

                <button class="big">
                    <a class="a" href="/">
                        { "Go home" }
                    </a>
                </button>
            </div>
        </>
    }
}
