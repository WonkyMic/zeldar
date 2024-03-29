use yew::prelude::*;
use stylist::{Style, style, yew::styled_component};
use crate::components::tile::Tile;

#[styled_component(Dashboard)]
pub fn dashboard() -> Html {
    let stylesheet: Style = style!(
        r#"
            .grid-container {
                display: grid;
                grid-template-columns: repeat(3, 1fr);
                gap: 10px;
                grid-auto-rows: minmax(100px, auto);
            }
        "#
    ).unwrap();
    html!{
        <div class={stylesheet}>
            <div class="grid-container">
                <Tile title="title0" />
                <Tile title="title1" />
                <Tile title="title2" />
                <Tile title="title3" />
                <Tile title="title4" />
                <Tile title="title5" />
            </div>
        </div>
    }
}