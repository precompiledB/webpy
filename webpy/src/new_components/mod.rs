pub mod instructions {
    use gloo::{console::debug, net::http::Request};
    use shared_structs::tasks::Assignment;
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    pub struct Instructions {
        current_lesson: i32,
        assignment: Assignment,
        expanded_tasks: Vec<bool>,
        node_ref: Vec<NodeRef>,
    }

    pub enum InstructionsMsg {
        NextInstruction,
        ReceivedAssignment(Assignment),
        Pressed(usize),
    }

    impl Component for Instructions {
        type Message = InstructionsMsg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                current_lesson: 0,
                assignment: Assignment::create_stub(),
                expanded_tasks: vec![],
                node_ref: vec![],
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let (symbol, color) = match self.assignment.status {
                shared_structs::tasks::Status::Complete => ("[x]", "gray"),
                shared_structs::tasks::Status::Current => ("[.]", "white"),
                shared_structs::tasks::Status::Locked => ("###", "darkgray"),
            };

            let tasks = self
                .assignment
                .tasks
                .iter()
                .enumerate()
                .map(|(idx, t)| {
                    let style = if self.expanded_tasks[idx] {
                        let scroll_height =
                            self.node_ref[idx].cast::<HtmlDivElement>().map_or_else(
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
                        let ctx = ctx.clone();
                        move |_| {
                            ctx.link().send_message(InstructionsMsg::Pressed(idx))
                        }
                    };

                    html! {
                        <div>
                            <button class="collapsible" onclick={onclick} ></button>
                            <div class="content" style={ style } ref={&self.node_ref[idx]}>
                                <p>{ &t.description }</p>
                                <p>{ &t.info }</p>
                            </div>
                        </div>
                    }
                })
                .collect::<Html>();

            html! {
                <div class="instructions">

                { &self.assignment.description }
                <div style={format!("color: {color};")}> {symbol} </div>

                { tasks }

                </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                InstructionsMsg::NextInstruction => {
                    let request = Request::get(&format!("assets/task{}.toml", self.current_lesson));

                    ctx.link().send_future(async {
                        let assignment = request
                            .send()
                            .await
                            .expect("Couldn't get data")
                            .json::<Assignment>()
                            .await
                            .expect("Couldn't transform into Assignment");

                        InstructionsMsg::ReceivedAssignment(assignment)
                    });

                    false
                }
                InstructionsMsg::ReceivedAssignment(assignment) => {
                    self.assignment = assignment;
                    self.expanded_tasks = (0..self.assignment.tasks.len()).map(|_| false).collect();

                    self.node_ref = (0..self.assignment.tasks.len())
                        .map(|_| NodeRef::default())
                        .collect();

                    true
                }
                InstructionsMsg::Pressed(idx) => {
                    self.expanded_tasks[idx] = !self.expanded_tasks[idx];
                    true
                }
            }
        }
    }
}

pub mod textinput {
    use gloo::{console::debug, net::http::Request};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen]
        fn editor_create();
        #[wasm_bindgen]
        fn editor_val() -> String;
        #[wasm_bindgen]
        fn editor_clr();
    }

    #[derive(Properties, PartialEq)]
    pub struct TextInputProps {
        pub is_submit: bool,
        pub is_clear: bool,
        pub onsubmitsuccess: Callback<String, ()>,
    }

    #[function_component]
    pub fn TextInput(props: &TextInputProps) -> Html {
        let div_ref = use_node_ref();

        {
            let div_ref = div_ref.clone();

            use_effect_with_deps(
                |div_ref| {
                    let div = div_ref
                        .cast::<HtmlDivElement>()
                        .expect("Not an html div element :(");

                    div.set_id("editor");
                    
                    editor_create();
                    
                    debug!("Added child");
                },
                div_ref,
            );

            if props.is_submit {

                let userinput = editor_val();

                let request = Request::post("execute_python").body(userinput);

                let callback = props.onsubmitsuccess.clone();

                wasm_bindgen_futures::spawn_local(
                    async move {
                        let t = request.send().await.expect("Couldn't fetch the request").text().await.expect("Couldn't read the Response");
                        callback.emit(t);
                    }
                );
            }

            if props.is_clear {
                editor_clr();
            }
        }

        html!(
            <div class="textinput">
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
            Self { data: "   ".into() }
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
