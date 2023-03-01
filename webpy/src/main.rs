use std::rc::Rc;

use gloo::console::debug;
use gloo::net::http::Request;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use shared_structs::tasks::Assignment;
use web_sys::console::log_1;
use web_sys::HtmlDivElement;
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
    let assignment = use_state_eq(|| Assignment::create_stub());
    let current_assignment = use_state_eq(|| 0);
    let current_lesson = use_state_eq(|| 0);
    let text = use_state(|| String::from("Output"));

    let on_lesson_changed = {
        let current_lesson = current_lesson.clone();
        Callback::from(move |x| {
            current_lesson.set(x);
        })
    };

    let onsubmitsuccess = {
        let text = text.clone();
        Callback::from(move |x: String| {
            debug!("Received ", &x);
            text.set(String::from(&x));
    })};

    let onsubmit = {
        let current_assignment = current_assignment.clone();
        let current_lesson = current_lesson.clone();
        Callback::from(move |_| {
            let userinput = editor_val();

            //let node_ref = instref.clone();
            //let div: HtmlDivElement = node_ref.cast::<HtmlDivElement>().expect("Couldn't get the div");
            let current = [("assignment", current_assignment.to_string()), ("lesson", current_lesson.to_string())];

            let request = Request::post("/execute_python")
                .query(current)
                .body(userinput);

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
        })
    };

    let onclear = |_| {
        editor_clr();
    };

    let onadvance = {
        let current_assignment = current_assignment.clone();
        let assignment = assignment.clone();
        move |_| {
            let current_assignment = current_assignment.clone();
            let request = Request::get(&format!("assets/task{}.toml", *current_assignment));
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
                current_assignment.set(*current_assignment + 1);
            });
        }
    };
    debug!(format!("assignment: {assignment:?}"));
    let assignment = assignment.clone();
    let text = &*text.clone();

    html! {
        <div class="root">
            <Instructions assignment={(*assignment).clone()} {on_lesson_changed}/>
            <TextInput/>
            <OutputTerminal text={ String::from(text) }/>
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
