use seed::{prelude::*, *};

use shared_structs::tasks::Assignment;
use web_sys::HtmlDivElement;

pub struct Model {
    current_lesson: i32,
    // TODO: change to task view struct later if necessary
    lesson_text: Assignment,
    is_completed: bool, // status
    is_pressed: Vec<bool>,
    el_ref: ElRef<HtmlDivElement>,
    scroll_height: i32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            current_lesson: 0,
            lesson_text: Assignment::create_stub(),
            is_completed: false,
            is_pressed: vec![],
            el_ref: ElRef::default(),
            scroll_height: 20,
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    NextInstruction,
    ReceiveView(String),
    Press(usize)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    web_sys::console::log_1(&format!("UPDATE {:?}", msg).into());
    match msg {
        Msg::NextInstruction => {
            orders.skip();

            let request = Request::new(format!("assets/task{}.toml", model.current_lesson));
            //let request = Request::new(format!("assets/{}.txt", model.current_lesson));

            orders.perform_cmd(async {
                let data = fetch(request).await.expect("Couldn't get data");
                let text = data.check_status().unwrap().text().await.unwrap();
                Msg::ReceiveView(text)
            });

            model.current_lesson = match model.current_lesson {
                0 => 1, 1 => 0, _ => 99,
            };
        }
        Msg::ReceiveView(payload) => {
            let ass = Assignment::from_toml(&payload).unwrap();
            model.lesson_text = ass;
            model.is_pressed = (0..model.lesson_text.tasks.len()).map(|_| false).collect::<Vec<_>>();
            orders.render();
        }
        Msg::Press(idx) => {
            model.is_pressed[idx] = !model.is_pressed[idx];

            let scroll_height = model.el_ref.get().and_then(|x| Some(x.scroll_height())).unwrap_or(200);
            model.scroll_height = scroll_height;

            debug_1(&format!("SH is {scroll_height}", ).into());
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn change_scroll_height(is_pressed: bool);
}

pub fn view(model: &Model) -> Node<Msg> {
    debug_1(&format!("{:?}", &model.lesson_text).into());

    let (symbol, color) = match model.lesson_text.status {
        shared_structs::tasks::Status::Complete => ("[x]","gray"),
        shared_structs::tasks::Status::Current => ("[.]", "white"),
        shared_structs::tasks::Status::Locked => ("###","darkgray"),
    };

    let tasks = model.lesson_text.tasks.iter().enumerate().map(|(idx, t)|{
        /* style![St::Display => match model.is_pressed[idx] {
            true => "block",
            false => "none",
        }], */

        let content = div![
            C!["content"], 
            if model.is_pressed[idx] {
                style![St::MaxHeight => format!("{}px", model.scroll_height), St::Transition => "max-height 0.5s ease-in"]
                //"max-height: 0; transition: max-height 0.5s ease-out;"
            } else {
                style![St::MaxHeight => 0, St::Transition => "max-height 0.5s ease-out"]
                //"max-height: 200px; transition: max-height 0.5s ease-in;"
            },
            p![&t.description],
            p![&t.info],
            el_ref(&model.el_ref),
        ];

        div![
            button![
                C!["collapsible", IF!(model.is_pressed[idx] => "active")],
                idx,
                ev(Ev::Click, move |_| Msg::Press(idx))
            ],
            content,
        ]
    });

    div![
        C!["instructions"],
        &model.lesson_text.description,
        div![attrs!(At::Color => color), symbol],
        tasks
    ]
}
