use yew::prelude::*;
use gloo::dialogs::prompt;

#[derive(Properties, PartialEq)]
pub struct ProfileManagerProps {
    pub name: String,
    pub onnamechange: Callback<String, ()>,
}

#[function_component(ProfileManager)]
pub fn profile_manager(props: &ProfileManagerProps) -> Html {
    let onclick = {
        let cb = props.onnamechange.clone();
        Callback::from(move |_me| {
            let name = prompt("Type in your user name", None).unwrap();
            cb.emit(name);
        })
    };

    html! {
        <div class="profile" {onclick}>
            { props.name.clone() }
        </div>
    }
}
