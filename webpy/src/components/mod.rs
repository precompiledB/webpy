pub mod user_profile;

pub mod instructions {
    use gloo::console::debug;
    use web_sys::HtmlDivElement;
    use yew::prelude::*;
    use shared_structs::tasks::{Assignment, Task};

    #[derive(Properties, PartialEq)]
    pub struct InstructionProps {
        pub assignment: Assignment,
        pub on_lesson_changed: Callback<i32, ()>,
    }

    #[function_component(Instructions)]
    pub fn instructions(props: &InstructionProps) -> Html {
        let current_expand = use_state(|| -1);

        let (symbol, color) = match props.assignment.status {
            shared_structs::tasks::Status::Complete => ("[x]", "gray"),
            shared_structs::tasks::Status::Current => ("[.]", "white"),
            shared_structs::tasks::Status::Locked => ("###", "darkgray"),
        };

        let on_lesson_changed = {
            let current_expand = current_expand.clone();
            let on_lesson_changed = props.on_lesson_changed.clone();
            Callback::from(move |idx| {
                current_expand.set(idx);
                on_lesson_changed.emit(idx)}
            )
        };
        let tasks = props.assignment.tasks.clone()
            .into_iter()
            .enumerate()
            .map(|(idx, task)| {
                let on_lesson_changed = on_lesson_changed.clone();
                let idx = idx as i32;
                html! {<VTask {task} {idx} {on_lesson_changed} is_expanded={idx == *current_expand}/>}
            })
            .collect::<Html>();

        html! {
            <div class="instructions">
                { &props.assignment.description }
                <div style={format!("color: {color};")}> {symbol} </div>

                { tasks }
            </div>
        }
    }

    #[derive(PartialEq, Properties)]
    pub struct VTaskProps {
        task: Task,
        is_expanded: bool,
        idx: i32,
        on_lesson_changed: Callback<i32, ()>,
    }

    fn v_task_style(is_expanded: bool, scroll_height: i32) -> String {
        if is_expanded {
            format!("max-height: {scroll_height}px; transition: max-height 0.15s ease-out;")
        } else {
            format!("max-height: 0px; transition: max-height 0.15s ease-in;")
        }
    }

    #[function_component(VTask)]
    fn v_task(props: &VTaskProps) -> Html {
        let node_ref = use_node_ref();

        let scroll_height = node_ref.cast::<HtmlDivElement>().map_or_else(
            || {
                debug!("Error getting scroll_height");
                20
            },
            |div| div.scroll_height(),
        );

        let style = v_task_style(props.is_expanded, scroll_height);

        let on_click = {
            let callback = props.on_lesson_changed.clone();
            let idx = props.idx.clone();
            Callback::from(move |_| {
                callback.emit(idx);
            })
        };

        html! {
            <div>
                <button class={classes!("collapsible", props.is_expanded.then(|| "active"))} onclick={on_click} >
                    if props.is_expanded {
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

    #[derive(yew::Properties, PartialEq)]
    pub struct OutputTerminalProps {
        pub text: AttrValue,
    }

    #[yew::function_component]
    pub fn OutputTerminal(props: &OutputTerminalProps) -> Html {
        let (color, text) = props.text.split_at(4);

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

pub mod profile_manager;


