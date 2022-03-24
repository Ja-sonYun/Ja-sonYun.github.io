use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn root_route(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! {
            <>
                <p>{ "About" }</p>
            </>
        },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        // After update 0.20.0 or after 0.19.0, check basename option on BrowserRouter
        <BrowserRouter>
            <Switch<Route> render={Switch::render(root_route)} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}
