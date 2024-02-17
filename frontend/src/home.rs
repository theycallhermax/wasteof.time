use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <div class="container">
                <h1>
                    { "Get ready to self-host the " }
                    <span class="highlight">{ "next generation social media." }</span>
                </h1>
                <p>{ "wasteof.time is a open-source version of the wasteof.money social platform." }</p>
                
                <br />

                <small>{ "Buttons coming soon." }</small>
            </div>
        </>
    }
}
