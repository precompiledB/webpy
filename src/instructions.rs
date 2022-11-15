use seed::{prelude::*, *};

pub struct Model {
    current_lesson: i32,
    // TODO: change to task view struct later if necessary
    lesson_text: String,
    is_completed: bool, // status
}

impl Model {
    pub fn new() -> Self {
        Self {
            current_lesson: 0,
            lesson_text: String::from("TEST"),
            is_completed: false,
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    NextInstruction,
    ReceiveView(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    web_sys::console::log_1(&format!("UPDATE {:?}", msg).into());
    match msg {
        Msg::NextInstruction => {
            orders.skip();

            let request = Request::new(format!("assets/{}.txt", model.current_lesson));

            orders.perform_cmd(async {
                let data = fetch(request).await.expect("Couldn't get data");
                let text = data.check_status().unwrap().text().await.unwrap();
                Msg::ReceiveView(text)
            });

            model.current_lesson = match model.current_lesson {
                0 => 1, 1 => 0, _ => 99,
            };
        }
        Msg::ReceiveView(text) => {
            model.lesson_text = text;
            orders.render();
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![C!["instructions"], model.lesson_text.clone()]
}
