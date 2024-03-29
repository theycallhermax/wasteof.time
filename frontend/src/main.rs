use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/about")]
    About,

    #[at("/users/:username")]
    // #[at("/@:username")]
    User { username: String },


    #[not_found]
    #[at("/404")]
    NotFound
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::Home /> },
        Route::About => html! { <pages::About /> },
        Route::User { username } => html! { <pages::User username={ username } /> },

        Route::NotFound => html! { <pages::errors::NotFound /> }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="header">
                <h2 class="span">
                    <a class="a" href="/">
                        { "wasteof.time" }
                    </a>
                </h2>

                <div class="end">
                    <h3>{ "explore" }</h3>
                    // <h3 class="sides">{ "join" }</h3>
                    // <h3 class="sides">{ "login" }</h3>
                </div>  
            </div>

            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
