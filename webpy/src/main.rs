use std::rc::Rc;

use gloo::console::debug;
use yew::prelude::*;
use web_sys::console::log_1;

mod new_components;

use new_components::instructions::Instructions;
use new_components::textinput::TextInput;

use yew::{function_component, html, Html, Properties};

#[function_component]
fn OutputTerminal() -> Html { html! {<div>{ "Hello :)" }</div>} }

#[function_component]
fn App() -> Html {
    let is_submit = use_state(|| false);
    let is_clear = use_state(|| false);

    let onsubmit = {
        let is_submit = is_submit.clone();
        move |_| {
            is_submit.set(true);
        }
    };


    let onclick = |_| log_1(&"Click".into());



    let onsubmitsuccess = |x| {
        debug!("Received ", x);
    };

    html! {
        <div>
            <Instructions/>
            <TextInput is_submit={*is_submit} is_clear={*is_clear} {onsubmitsuccess}/>
            <OutputTerminal/>
            <button onclick={onclick}>{ "Advance" }</button>
            <button onclick={onclick}>{ "Clear" }</button>
            <button onclick={onsubmit}>{ "Submit" }</button>
        </div>
    }
}

/*
            instructions::view(&model.instructions).map_msg(Msg::Instructions),
        //textinput::view(&model.textinput).map_msg(Msg::TextInput),
        output_terminal::view(&model.output_terminal).map_msg(Msg::OutputTerminal),
        div![
            C!("butt"),
            button![
                "Advance",
                C!("advancebutt"),
                ev(Ev::Click, |_| Msg::Instructions(instructions::Msg::NextInstruction)),
            ],
            button![
                "Clear",
                C!("clearbutt"),
                ev(Ev::Click, |_| Msg::TextInput(textinput::Msg::Clear)),
            ],
            button![
                "Submit",
                C!("submitbutt"),
                ev(Ev::Click, |_| Msg::TextInput(textinput::Msg::Submit)),

 */

fn main() {
    yew::Renderer::<App>::new().render();
}