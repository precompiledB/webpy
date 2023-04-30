use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct UserData {
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct ProfileManagerProps {
    pub user_data: UserData,
    pub on_user_change: Callback<MouseEvent, ()>,
}

#[function_component(ProfileManager)]
pub fn profile_manager(props : &ProfileManagerProps) -> Html {
    let onclick = props.on_user_change.clone();
    html! {
        <div class="user" {onclick}>
            { props.user_data.name.clone() }
        </div>
    }
}