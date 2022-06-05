use yew::prelude::*;

// todo :: fix import stuttering 
use crate::components::tile::Tile;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            // todo :: dynamic generation and formatting
            <Tile />
            <Tile />
        </div>
    }
}