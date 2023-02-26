pub mod instructions {
    use gloo::{console::debug, net::http::Request};
    use shared_structs::tasks::Assignment;
    use web_sys::HtmlDivElement;
    use yew::prelude::*;

    #[derive(yew::Properties, PartialEq, Eq)]
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
    }

    impl Component for Instructions {
        type Message = InstructionsMsg;
        type Properties = InstructionProps;

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                //assignment: Assignment::create_stub(),
                expanded_tasks: vec![],
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
                        let ctx = ctx.link().clone();
                        move |_| ctx.send_message(InstructionsMsg::Pressed(idx))
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

                { &ctx.props().assignment.description }
                <div style={format!("color: {color};")}> {symbol} </div>

                { tasks }

                </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            let ass = &ctx.props().assignment;
            debug!(format!("Updating assignment:\n{ass:?}"));

            //if &self.assignment != ass {
            //debug!("Changed");
            //ctx.props().assignment = ass.clone();

            self.expanded_tasks = (0..ctx.props().assignment.tasks.len())
                .map(|_| false)
                .collect();
            /*
             */
            //}
            match msg {
                InstructionsMsg::Pressed(idx) => {
                    debug!(idx);
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
