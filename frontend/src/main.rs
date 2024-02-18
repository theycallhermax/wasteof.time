use yew::prelude::*;
use yew_router::prelude::*;

// components
#[path = "components/banned.rs"] mod banned;
use banned::Banned;

// pages
#[path = "pages/home.rs"] mod home;
use home::Home;
#[path = "pages/about.rs"] mod about;
use about::About;
#[path = "pages/user.rs"] mod user;
use user::User;

// error pages
#[path = "pages/404.rs"] mod notfound;
use notfound::NotFound;

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
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::User { username } => html! { <User username={ username } /> },

        Route::NotFound => html! { <NotFound /> }
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
