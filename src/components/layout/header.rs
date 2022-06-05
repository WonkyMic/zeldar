use yew::prelude::*;
use yew_router::prelude::Link;
// use yew_router::history::History;
use stylist::{style, yew::styled_component};

use crate::router::Route;

// #[function_component(Header)]
#[styled_component(Header)]
pub fn header() -> Html {
    // let history = use_history().unwrap();

    // let onclick_callback = Callback::from(move |_| history.push(Route::Home));

    let stylesheet = style!(
        r#"
            ul {
                list-style-type: none;
                margin: 0;
                padding: 0;
                overflow: hidden;
                background-color: #333;
            }
            
            li {
                float: left;
            }
            
            li a {
                display: block;
                color: white;
                text-align: center;
                padding: 14px 16px;
                text-decoration: none;
            }
            
            /* Change the link color to #111 (black) on hover */
            li a:hover {
                background-color: #111;
            }

            .active { 
                background-color: #04AA6D;
            }
        "#
    ).unwrap();
    html!{
        <div class={stylesheet}>
            <nav class="navbar">
                <ul>
                    <li>
                        <Link<Route> to={Route::Home} classes="nav-link">
                            { "Home" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Hello} classes="nav-link">
                            { "Hello" }
                        </Link<Route>>
                    </li>
                </ul>
            </nav>
        </div>
    }
}