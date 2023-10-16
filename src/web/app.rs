use yew::prelude::*;

use crate::core::encoding::Guldhjerte;
#[function_component]
pub fn App() -> Html {
    let test = "1234.567890".to_string().to_guldhjerte().unwrap();

    html! {
        <div>
            <p>{"Guldhjerte: "}{ &test }</p>
        </div>
    }
}
