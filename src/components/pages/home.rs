use yew::prelude::*;

// todo :: fix import stuttering 
use crate::components::pages::dashboard::Dashboard;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <Dashboard />
        </div>
    }
}