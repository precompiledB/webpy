use seed::{prelude::*, *};

pub struct Model {
    data: String,
}

impl Model {
    pub fn new() -> Model {
       Model {
            data: "".into(),
       } 
    }
}

const BUSY_DATA: &str = "BUSY :(";

pub enum Msg {
    ShowBusy,
    ShowOutput(String), // change string to a struct
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ShowBusy => model.data = BUSY_DATA.to_owned(),
        Msg::ShowOutput(response) => model.data = response,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div!(
        C!["terminal"],
        //attrs![At::from("readonly") => AtValue::None],
        model.data.clone()
    )
}
