use seed::{prelude::*, *};

pub struct Model {
    data: String,
}

impl Model {
    pub fn new() -> Model {
       Model {
            data: "     ".into(),
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
    let (output_info, text) = (&model.data[..4], &model.data[4..]);
    let color = match output_info {
        "Suc:" => "color:green",
        "Err:" => "color:red",
        "Inf:" => "color:gray",
        _ => "color:cauliflowerblue",
    };

    div!(
        C!["terminal"],
        //attrs![At::from("readonly") => AtValue::None],
        attrs![At::Style => color],
        text
    )
}
