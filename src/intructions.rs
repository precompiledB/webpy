use seed::prelude::*;

pub struct Model {
    current_lesson: i32,
    // TODO: change to task view struct later if necessary
    lesson_text: String,
    is_completed: bool, // status 
}

impl Model {
    pub fn new() -> Self { Self { current_lesson: 0, lesson_text: String::from(""), is_completed: false } }
}

pub enum Msg {
    NextInstruction,
    RefreshView,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::NextInstruction => {
            orders.skip();
            let text;
            let request = Request::new(format!("{}.txt", model.current_lesson));
            orders.perform_cmd(async move {
                let data = fetch(request).await.expect("Couldn't get data");
                text = data.check_status().unwrap().text().await.unwrap();
            });
            model.lesson_text = text;
        },
        Msg::RefreshView => {
            orders.render();
        }
    }
}
