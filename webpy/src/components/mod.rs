pub mod instructions {
    use std::{ops::AddAssign, rc::Rc};

    use gloo::{console::debug, net::http::Request};
    use shared_structs::tasks::{Assignment, Task};
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    #[derive(PartialEq, Properties)]
    pub struct VTaskProps {
        task: Task,
        idx: i32,
        on_lesson_changed: Callback<i32, ()>,
    }

    // Make VTask radiobuttons so that their is only one current expanded selcetion

    #[function_component]
    fn VTask(props: &VTaskProps) -> Html {
        let is_expanded = use_state(|| false);
        let node_ref = use_node_ref();

        let style = if *is_expanded {
            let scroll_height = node_ref.cast::<HtmlDivElement>().map_or_else(
                || {
                    debug!("Error getting scroll_height");
                    20
                },
                |div| div.scroll_height(),
            );

            format!("max-height: {scroll_height}px; transition: max-height 0.15s ease-out;")
        } else {
            format!("max-height: 0px; transition: max-height 0.15s ease-in;")
        };

        let idx = props.idx;

        let onclick = {
            let is_expanded = is_expanded.clone();
            let on_lesson_changed = props.on_lesson_changed.clone();
            move |_| {
                if !*is_expanded {
                    on_lesson_changed.emit(idx);
                }
                is_expanded.set(!*is_expanded);
            }
        };

        let is_expanded = is_expanded.clone();

        html! {
            <div>
                <button class={classes!("collapsible", is_expanded.then(|| "active"))} onclick={onclick} >
                    if *is_expanded {
                        { "- " }
                    } else {
                        { "+ " }
                    }
                    { &props.task.description }
                </button>
                <div class="content" style={ style } ref={node_ref}>
                    <p>{ &props.task.info }</p>
                    <p>{ "Help: " }{ &props.task.help }</p>
                </div>
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct InstructionProps {
        pub assignment: Assignment,
        pub on_lesson_changed: Callback<i32, ()>,
    }

    pub struct Instructions {
        //assignment: Assignment,
        //expanded_tasks: Vec<bool>,
        current: i32,
    }

    pub enum InstructionMsg {
        ChangedLesson(i32),
    }

    impl Component for Instructions {
        type Message = InstructionMsg;
        type Properties = InstructionProps;

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                //assignment: Assignment::create_stub(),
                //expanded_tasks: Vec::with_capacity(128),
                current: 0,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (symbol, color) = match ctx.props().assignment.status {
                shared_structs::tasks::Status::Complete => ("[x]", "gray"),
                shared_structs::tasks::Status::Current => ("[.]", "white"),
                shared_structs::tasks::Status::Locked => ("###", "darkgray"),
            };

            let link = ctx.link().clone();

            let on_lesson_changed =
                Callback::from(move |x: i32| link.send_message(InstructionMsg::ChangedLesson(x)));

            let tasks = ctx
                .props()
                .assignment
                .tasks
                .clone()
                .into_iter()
                .enumerate()
                .map(|(idx, task)| {
                    html! { <VTask idx={idx as i32} {task} on_lesson_changed={on_lesson_changed.clone()}/> }
                })
                .collect::<Html>();

            ctx.props().on_lesson_changed.emit(self.current);

            html! {
                <div class="instructions" data-current={self.current.to_string()}>

                { &ctx.props().assignment.description }
                <div style={format!("color: {color};")}> {symbol} </div>

                { tasks }

                </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                InstructionMsg::ChangedLesson(num) => self.current = num,
            }

            true
        }
    }
}

pub mod textinput {
    use gloo::{console::debug, net::http::Request};
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    use crate::components::interop::editor_create;

    #[function_component]
    pub fn TextInput() -> Html {
        let div_ref = use_node_ref();

        {
            let div_ref = div_ref.clone();

            use_effect_with_deps(
                |div_ref| {
                    let div = div_ref
                        .cast::<HtmlDivElement>()
                        .expect("Not an html div element :(");

                    div.set_id("editor");
                    div.set_inner_text(r#"print("Hello World :)")"#);

                    editor_create();

                    debug!("Added child");
                },
                div_ref,
            );
        }

        html!(
            <div class={classes!("textinput", "webpyinterface")}>
                <div ref={ div_ref }></div>
            </div>
        )
    }
}

pub mod output_terminal {
    use yew::prelude::*;

    pub struct OutputTerminal {
        data: String,
    }

    pub enum OutputTerminalMsg {
        ShowBusy,
        ShowOutput(String), // change to a struct
    }

    impl Component for OutputTerminal {
        type Message = OutputTerminalMsg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                data: "    ".into(),
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                OutputTerminalMsg::ShowBusy => self.data = String::from("BUSY"),
                OutputTerminalMsg::ShowOutput(resp) => self.data = resp,
            }
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (color, text) = self.data.split_at(4);

            let color = match color {
                "Suc:" => "green",
                "Err:" => "red",
                "Inf:" => "gray",
                _ => "cauliflowerblue",
            };

            html! {
                <div class="terminal" style={ format!("color: {color}") }>
                    { text }
                </div>
            }
        }
    }
}

pub mod interop {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen]
        pub fn editor_create();
        #[wasm_bindgen]
        pub fn editor_val() -> String;
        #[wasm_bindgen]
        pub fn editor_clr();
    }
}
