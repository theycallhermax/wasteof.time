use yew::prelude::*;

#[function_component]
pub fn Banned() -> Html {
    html! {
        <div class="container">
            <h1>
                { "You are currently " }
                <span class="highlight red">
                    if false { { "temporaily " } }
                    { "banned " }
                </span>
                { "from wasteof.time" }
            </h1>
            <p>{ "Please note that if you attempt to evade this ban with alternate accounts, your ban may deteriorate in an more severe punishment." }</p>

            if true { // todo: replace "true" with the real banned boolean
                <div class="container border">
                    <h3>{ "Ban reason" }</h3>
                    <p>{ "Lorem ispum dolor sit amet" }</p>

                    <br />

                    <small>{ "If this ban reason seems incorrect or invalid, please notify us immediately" }</small>
                </div>
            }

            <button>{ "Log out" }</button>
        </div>
    }
}
