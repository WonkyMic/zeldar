mod components;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::footer::Footer;
use crate::components::layout::header::Header;
use crate::router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
            <Footer />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}