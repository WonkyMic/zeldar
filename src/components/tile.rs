use yew::prelude::*;
use stylist::{Style, style, yew::styled_component};

#[styled_component(Tile)]
pub fn tile() -> Html {
    let stylesheet: Style = style!(
        r#"
            #outline {
                height: 180px;
                width: 360px;
                border: 2px solid #73AD21;
                border-radius: 25px;
                padding: 20px;
                box-shadow: 10px 10px 5px #e1fae2;
            }
            #title {
                text-align: center;
                border-bottom-style: solid;
                border-color: #73AD21;
                border-width: 2px;
            }
        "#
    ).unwrap();
    html!{
        <div class={stylesheet}>
            <div id="outline">
                <div id="title">{"Placeholder Title"}</div>
            </div>
        </div>
    }
}