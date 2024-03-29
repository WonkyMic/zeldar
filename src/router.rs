use crate::components::pages::home::Home;
use crate::components::pages::hello::Hello;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!{ <Home /> },
        Route::Hello => html!{ <Hello /> },
    }
}