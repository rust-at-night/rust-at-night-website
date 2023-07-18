pub mod components;

use components::posts::Posts;
use yew::prelude::*;
use yew_router::prelude::*;

/// Frontend router.
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/posts")]
    Posts,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "🌃 Rust at Night" }</h1> },
        Route::Posts => html! { <Posts /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
