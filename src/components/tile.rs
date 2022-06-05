use yew::prelude::*;
use stylist::{Style, style, yew::styled_component};

#[styled_component(Tile)]
pub fn tile() -> Html {
    let stylesheet: Style = style!(
        r#"
            #outline {
                height: 180px;
                width: 360px;
                padding: 20px;
                border-radius: 5px;
                background-image:
                    linear-gradient(to left, #316e2f 60% , transparent),
                    linear-gradient(to left, #316e2f 60% , transparent),
                    linear-gradient(to right, #316e2f 60% , transparent),
                    linear-gradient(to right, #316e2f 60% , transparent),
                    linear-gradient(transparent, #316e2f 60%),
                    linear-gradient(transparent, #316e2f 60%),
                    linear-gradient(#316e2f 60%, transparent), 
                    linear-gradient(#316e2f 60%, transparent);
                background-size:
                    4em .5em,
                    4em .5em,
                    4em .5em,
                    4em .5em,
                    .5em 4em, 
                    .5em 4em, 
                    .5em 4em, 
                    .5em 4em;
                background-position:
                    21em 0,
                    21em 13.25em,
                    0 13.25em,
                    0 0,
                    0 10em,
                    24.5em 10em,
                    24.5em 0,
                    0 0;
                background-repeat: no-repeat;
            }
            #outline:hover {
                box-shadow: 10px 10px 5px #80a87e;
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