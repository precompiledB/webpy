use std::rc::Rc;

use gloo::console::debug;
use gloo::net::http::Request;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use shared_structs::tasks::Assignment;
use web_sys::console::log_1;
use yew::platform::time::sleep;
use yew::prelude::*;

mod components;

use components::instructions::Instructions;
use components::output_terminal::OutputTerminal;
use components::textinput::TextInput;

use yew::{function_component, html, Html};

use crate::components::interop::{editor_clr, editor_val};

#[function_component]
fn App() -> Html {
    let current_lesson = use_state(|| 0);
    let assignment = use_state_eq(|| Assignment::create_stub());
    
    let onsubmitsuccess = Callback::from(|x| {
        debug!("Received ", x);
    });

    let onsubmit = Callback::from(move |_| {
        let userinput = editor_val();

        let request = Request::post("/execute_python").body(userinput);
        
        let callback = onsubmitsuccess.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
            let t = request
            .send()
                .await
                .expect("Couldn't fetch the request")
                .text()
                .await
                .expect("Couldn't read the Response");
            callback.emit(t);
        });
    });

    let onclear = |_| {
        editor_clr();
    };
    
    let onadvance = {
        let current_lesson = current_lesson.clone();
        let assignment = assignment.clone();
        move |_| {
            let request = Request::get(&format!("assets/task{}.toml", *current_lesson));
            let assignment = assignment.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let text = request
                    .send()
                    .await
                    .expect("Couldn't get data")
                    .text()
                    .await
                    .expect("lol");
                debug!(&text);
                let ass = Assignment::from_toml(&text);

                debug!(format!("{ass:?}"));

                let ass = ass.expect("Couldn't transform into Assignment");
                assignment.set(ass);
            });
        }
    };
    debug!(format!("assignment: {assignment:?}"));
    let assignment = assignment.clone();
    html! {
        <div class="root">
            <Instructions assignment={(*assignment).clone()}/>
            <TextInput/>
            <OutputTerminal/>
            <div class="butt">
                <button class="advancebutt" onclick={onadvance}>{ "Advance" }</button>
                <button class="clearbutt" onclick={onclear}>{ "Clear" }</button>
                <button class="submitbutt" onclick={onsubmit}>{ "Submit" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
