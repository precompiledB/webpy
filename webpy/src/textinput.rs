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
            userinput: String::from(""),
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

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    web_sys::console::log_1(&format!("UPDATE {:?}", msg).into());
    
    match msg {
        Msg::Clear => {
            model.userinput = "".into();
        },
        Msg::TextChanged(s) => model.userinput = s,
        Msg::Submit => {
            orders.skip();
            let request = Request::new("execute_python")
                .method(Method::Post)
                .text(model.userinput.clone());

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
    textarea![
        C!["textinput", "editable"],
        attrs!(At::Value => model.userinput, At::Rows => 30, At::Cols => 40),
        input_ev(Ev::Input, Msg::TextChanged)
    ]
}
