pub mod instructions {
    use gloo::{console::debug, net::http::Request};
    use shared_structs::tasks::{Assignment, Task};
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct VTaskProps {
        task: Task,
    }

    #[function_component]
    fn VTask(props: &VTaskProps) -> Html {
        let is_expanded = use_state(|| false);
        let node_ref = use_node_ref();

        let style = if *is_expanded {
            let scroll_height =
                node_ref.cast::<HtmlDivElement>().map_or_else(
                    || {
                        debug!("Error getting scroll_height");
                        20
                    },
                    |div| div.scroll_height(),
                );

            format!(
                "max-height: {scroll_height}; transition: max-height 0.5s ease-out;"
            )
        } else {
            format!("max-height: 0px; transition: max-height 0.5s ease-in;")
        };

        let onclick = {
            move |_| is_expanded.set(!*is_expanded)
        };

        html! {

            <div>
                <button class="collapsible" onclick={onclick} ></button>
                <div class="content" style={ style } ref={node_ref}>
                    <p>{ &props.task.description }</p>
                    <p>{ &props.task.info }</p>
                </div>
            </div>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct InstructionProps {
        pub assignment: Assignment,
    }

    pub struct Instructions {
        //assignment: Assignment,
        expanded_tasks: Vec<bool>,
        node_ref: Vec<NodeRef>,
    }

    pub enum InstructionsMsg {
        Pressed(usize),
        ChangedAssignmentProps(Vec<bool>, Vec<NodeRef>),
    }

    impl Component for Instructions {
        type Message = InstructionsMsg;
        type Properties = InstructionProps;

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                //assignment: Assignment::create_stub(),
                expanded_tasks: Vec::with_capacity(128),
                node_ref: Vec::with_capacity(128),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (symbol, color) = match ctx.props().assignment.status {
                shared_structs::tasks::Status::Complete => ("[x]", "gray"),
                shared_structs::tasks::Status::Current => ("[.]", "white"),
                shared_structs::tasks::Status::Locked => ("###", "darkgray"),
            };

            let tasks = ctx
                .props()
                .assignment
                .tasks
                .clone()
                .into_iter()
                .map(|task| {
                    html!{ <VTask {task}/> }
                })
                .collect::<Html>();

            html! {
                <div class="instructions">

                { &ctx.props().assignment.description }
                <div style={format!("color: {color};")}> {symbol} </div>

                { tasks }

                </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            debug!("Update!");
            match msg {
                InstructionsMsg::Pressed(idx) => {
                    debug!(idx);
                    self.expanded_tasks[idx] = !self.expanded_tasks[idx];
                    true
                }
                InstructionsMsg::ChangedAssignmentProps(expanded_tasks, node_ref) => {
                    debug!(format!("{expanded_tasks:?}"));
                    self.expanded_tasks = expanded_tasks;
                    self.node_ref = node_ref;
                    true
                },
            }
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
