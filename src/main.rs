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
            <BrowserRouter>
                <Header />
                <Switch<Route> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}