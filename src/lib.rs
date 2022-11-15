use seed::{prelude::*, *, app::orders};

mod instructions;
mod textinput;

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> State {
    let instructions = instructions::Model::new();

    orders.send_msg(Msg::Instructions(instructions::Msg::NextInstruction));

    State {
        instructions,
        textinput: textinput::Model::new(),
    }
}

// `Model` describes our app state.
struct State {
    instructions: instructions::Model,
    textinput: textinput::Model,
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    Instructions(instructions::Msg),
    TextInput(textinput::Msg),

}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut State, orders: &mut impl Orders<Msg>) {
    match msg {
        //Msg::Increment => model.counter += 1,
        Msg::Instructions(m)=> {
            instructions::update(m, &mut model.instructions, &mut orders.proxy(Msg::Instructions));
        },
        Msg::TextInput(m) => {
            textinput::update(m, &mut model.textinput, &mut orders.proxy(Msg::TextInput));
        }
    }
}

// `view` describes what to display.
fn view(model: &State) -> impl IntoNodes<Msg> {
    nodes![
        
        instructions::view(&model.instructions).map_msg(Msg::Instructions),
        textinput::view(&model.textinput).map_msg(Msg::TextInput),
        button![
            "Advance",
            ev(Ev::Click, |_| Msg::Instructions(instructions::Msg::NextInstruction)),
        ],
        button![
            "Clear",
            ev(Ev::Click, |_| Msg::TextInput(textinput::Msg::Clear)),
        ],
        button![
            "Submit",
            ev(Ev::Click, |_| Msg::TextInput(textinput::Msg::Submit)),
        ]
        div![
            /* "This is a counter: ",
            C!["counter"],
            button![
                model.counter,
                ev(Ev::Click, |_| Msg::Increment),
            ], */
            textarea![
                id!["me"],
                //attrs!["rows" => 10],
                "text"
            ],
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
