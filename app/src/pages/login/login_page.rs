use yew::prelude::*;

use shared_components::*;
use core::translations::translate;
use super::{LoginState, LoginAction};
use crate::translations::use_translations;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let translations = use_translations();
    let login_state = use_context::<UseReducerHandle<LoginState>>().expect("no ctx found");

    let onclick = {
        let login_state = login_state.clone();
        Callback::from(move |_| login_state.dispatch(LoginAction::Login("asd".into())))
    };

    html! {
        <div>
            <Header variant={HeaderVariant::PageTitle}>
                {translate("login", &translations)}
            </Header>
            <Header>
                {translate("welcome", &translations)}
            </Header>
            <button {onclick}>
                {translate("login", &translations)}
            </button>
        </div>
    }
}