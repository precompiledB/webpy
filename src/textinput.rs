use seed::{prelude::*, *};

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
            let request = Request::new("something")
                .method(Method::Post)
                .text(model.userinput.clone());

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");
/* 
                if response.status().is_ok() {
                    ()
                } else {
                    Msg::SubmitFailed(response.status().text)
                } */
            });
        },
    }


}

pub fn view(model: &Model) -> Node<Msg> {
    textarea![
        C!["textinput"],
        attrs!(At::Value => model.userinput),
        input_ev(Ev::Input, Msg::TextChanged)
    ]
}
