use yew::prelude::*;

use super::{LoginState, LoginPage};

#[derive(Properties, PartialEq)]
pub struct RequireLoginProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RequireLogin)]
pub fn require_user_session(props: &RequireLoginProps) -> Html {
    let login_state = use_reducer(LoginState::default);

    html! {
        <ContextProvider<UseReducerHandle<LoginState>> context={login_state.clone()}>
            if !login_state.logged_in {
                <LoginPage />
            }
            else {
                { for props.children.iter() }
            }
        </ContextProvider<UseReducerHandle<LoginState>>>
    }
}