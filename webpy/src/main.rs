use std::rc::Rc;

use gloo::console::debug;
use gloo::net::http::Request;
use gloo::timers::callback::Timeout;
use gloo::utils::window;
use shared_structs::tasks::{Assignment, Status};
use web_sys::console::log_1;
use web_sys::HtmlDivElement;
use yew::platform::time::sleep;
use yew::prelude::*;

mod components;

use components::instructions::Instructions;
use components::output_terminal::OutputTerminal;
use components::textinput::TextInput;
use components::profile_manager::ProfileManager;

use yew::{function_component, html, Html};
use shared_structs::users::{AssignmentProgress, TaskProgress, User};

use crate::components::interop::{editor_clr, editor_val};
use crate::components::user_profile::UserData;

const DEFAULT_USER_NAME: &str = "NoUser";

#[function_component(App)]
fn app() -> Html {
    let user = User {
        name: "NoUser".into(),
        current_progress: vec![],
    };

    let user1 = User {
        name: "Leander".into(),
        current_progress: vec![
            AssignmentProgress {
                assignment: 0,
                task_progress: vec![
                    TaskProgress {
                        task: 2,
                        status: Status::Complete,
                    }
                ]
            }
        ],
    };

    debug!(serde_json::to_string(&vec![user, user1]).unwrap());

    //---------------------


    let assignment = use_state_eq(|| Assignment::create_stub());
    let current_assignment = use_state_eq(|| 0);
    let current_lesson = use_state_eq(|| -1);
    let text = use_state(|| String::from("Output"));

    use gloo::storage::{LocalStorage, Storage};

    let user_data = {
        if LocalStorage::length() > 0 {
            debug!("LocalStorage", LocalStorage::get_all::<String>().unwrap());
        } else {
            debug!("NoLocalStorage");
        }
        use_state(||
            UserData {
                name: DEFAULT_USER_NAME.into(),
            })
    };
    let text = use_state_eq(|| String::from("Output"));
    let profile_name = use_state_eq(|| String::from("NoUser"));

    let on_lesson_changed = {
        let current_lesson = current_lesson.clone();
        Callback::from(move |x| {
            current_lesson.set(x);
        })
    };

    let onsubmitsuccess = {
        let text = text.clone();
        Callback::from(move |x: String| {
            debug!("Received ", &x);
            text.set(String::from(&x));
    })};

    let onsubmit = {
        let current_assignment = current_assignment.clone();
        let current_lesson = current_lesson.clone();
        Callback::from(move |_| {
            if *current_lesson == -1 {
                gloo::dialogs::alert("Please select a task from the left panel first.");
            } else {

            let userinput = editor_val();

            let current = [("assignment", current_assignment.to_string()), ("task", current_lesson.to_string())];

            let request = Request::post("/execute_python")
                .query(current)
                .body(userinput);

            let callback = onsubmitsuccess.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let t = request
                    .send()
                    .await
                    .expect("Couldn't fetch the request")
                    .text()
                    .await
                    .expect("Couldn't read the Response");
                callback.emit(t);
            });
            }

        })
    };

    let onclear = |_| {
        editor_clr();
    };

    let onadvance = {
        let current_assignment = current_assignment.clone();
        let assignment = assignment.clone();
        move |_| {
            let current_assignment = current_assignment.clone();
            let request = Request::get(&format!("/assets/task{}.toml", *current_assignment));
            let assignment = assignment.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let text = request
                    .send()
                    .await
                    .expect("Couldn't get data")
                    .text()
                    .await
                    .expect("lol");
                debug!(&text);
                let ass = Assignment::from_toml(&text);

                debug!(format!("{ass:?}"));

                let ass = ass.expect("Couldn't transform into Assignment");
                assignment.set(ass);
                current_assignment.set(*current_assignment + 1);
            });
        }
    };

    let on_user_change = {
        let user_data = user_data.clone();
        Callback::from(move |_| {
            let requested_user = gloo::dialogs::prompt("Please type in a User name", Some("user name"))
                .unwrap_or(DEFAULT_USER_NAME.into());
            user_data.set(
                UserData {
                    name: requested_user
                }
            );
        })
    };


    let onnamechange = {
        let profile_name = profile_name.clone();
        Callback::from(move |s| {
            profile_name.set(s);
            debug!(&*(*profile_name).clone());
    })};

    debug!(format!("assignment: {assignment:?}"));
    let assignment = assignment.clone();
    let text = &*text.clone();

    html! {
        <div class="root">
            <ProfileManager name={profile_name.to_string()} {onnamechange} />
            <Instructions assignment={(*assignment).clone()} {on_lesson_changed}/>
            <TextInput/>
            <OutputTerminal text={ String::from(text) }/>
            <ProfileManager user_data={(*user_data).clone()}  {on_user_change}/>
            <div class="butt">
                <button class="advancebutt" onclick={onadvance}>{ "Advance" }</button>
                <button class="clearbutt" onclick={onclear}>{ "Clear" }</button>
                <button class="submitbutt" onclick={onsubmit}>{ "Submit" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
