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

                    html! {
                        <div>
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
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    #[function_component]
    fn TextInput() -> Html {
        let div_ref = use_node_ref();

        let div = div_ref.cast::<HtmlDivElement>().unwrap();

        

        html!(
            <div ref={ div_ref }></div>
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
