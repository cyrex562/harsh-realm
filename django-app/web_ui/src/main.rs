use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

#[function_component]
fn App() -> Html {
    let mut rows: Vec<String> = vec![];
    let onclick = {
       
        move |_| {
            log!("button clicked");
            rows.push("row".to_string());
        }
    };

    html! {
        <div>
            <div>
                <p>{"data table"}</p>
                <table>
                {
                    rows.iter().map(|row_string|{
                    html!{
                        <tr>
                            <td>{row_string}</td>
                        </tr>
                    }
                }).collect::<Html>()
            }
                </table>
            </div>
            <hr/>
            <div>
                <input type="text" />
                <button {onclick}>{"button"}</button>
            </div>
            <p>{"xyz"}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}