use seed::{prelude::*, *};

use web_sys::console::log_1 as consolelog;

pub struct Model {
    //preexisting_text: String,
    userinput: String,
}

impl Model {
    pub fn new() -> Self {
        Self {
            //preexisting_text: String::from("test"),
            userinput: String::from("print(\"Hello World :)\")"),
        }
    }
}

#[derive(Debug)]

pub enum Msg {
    //LoadCode(String),
    Clear,
    TextChanged(String),
    Submit,
    SubmitFailed(String),
    SubmitSuccesful(String), // TODO: change string to json/serde stuff
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn editor_val() -> String;
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    web_sys::console::log_1(&format!("UPDATE {:?}", msg).into());
    
    match msg {
        Msg::Clear => {
            model.userinput = "".into();
        },
        Msg::TextChanged(s) => model.userinput = s,
        Msg::Submit => {
            let userinput = editor_val();

            orders.skip();
            let request = Request::new("execute_python")
                .method(Method::Post)
                .text(userinput);

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");
 
                if response.status().is_ok() {
                    Msg::SubmitSuccesful(response.text().await.unwrap())
                } else {
                    Msg::SubmitFailed(response.status().text)
                }
            });
        }
        Msg::SubmitFailed(error_msg) => consolelog(&error_msg.into()),
        Msg::SubmitSuccesful(_response) => {},
    }


}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["textinput"],
        model.userinput.clone(),
        input_ev(Ev::Input, Msg::TextChanged)
    ]
}
