use seed::{prelude::*, *};

mod intructions;

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> State {
    State { 
        current_lesson: 1,
        user: UserProfile,
        text_input: String::from(""),
    }
}

struct UserProfile;

// `Model` describes our app state.
struct State { 
    current_lesson: u32,
    user: UserProfile,
    text_input: String,
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    Run,
    Break,
    NextLesson(u32),
    Login,
    Logout,
    Help,
    ClearTerminal,
    ShowResult,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut State, _: &mut impl Orders<Msg>) {
    match msg {
        //Msg::Increment => model.counter += 1,
        _ => {},
    }
}

// `view` describes what to display.
fn view(model: &State) -> Node<Msg> {
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
}

#[wasm_bindgen(start)]
pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
