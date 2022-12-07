use seed::{prelude::*, *};

mod instructions;
mod textinput;
mod output_terminal;

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> State {
    let instructions = instructions::Model::new();

    orders.send_msg(Msg::Instructions(instructions::Msg::NextInstruction));

    State {
        instructions,
        textinput: textinput::Model::new(),
        output_terminal: output_terminal::Model::new(),
    }
}

// `Model` describes our app state.
struct State {
    instructions: instructions::Model,
    textinput: textinput::Model,
    output_terminal: output_terminal::Model,
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    Instructions(instructions::Msg),
    TextInput(textinput::Msg),
    OutputTerminal(output_terminal::Msg),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut State, orders: &mut impl Orders<Msg>) {
    match msg {
        //Msg::Increment => model.counter += 1,
        Msg::Instructions(m)=> {
            instructions::update(m, &mut model.instructions, &mut orders.proxy(Msg::Instructions));
        },
        Msg::TextInput(textinput::Msg::SubmitSuccesful(response)) => {orders.send_msg(Msg::OutputTerminal(output_terminal::Msg::ShowOutput(response))); },
        Msg::TextInput(m) => {
            textinput::update(m, &mut model.textinput, &mut orders.proxy(Msg::TextInput));
        }
        Msg::OutputTerminal(m) => {
            output_terminal::update(m, &mut model.output_terminal, &mut orders.proxy(Msg::OutputTerminal))
        }
    }
}

// `view` describes what to display.
fn view(model: &State) -> impl IntoNodes<Msg> {
    nodes![
        
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
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
