use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::login::{LoginAction, LoginState, RequireLogin};
use crate::routes::Route;
use shared_components::*;
use core::translations::translate;
use crate::translations::use_translations;

#[function_component(OtherPage)]
pub fn home_page() -> Html {
    let translations = use_translations();
    let history = use_history().unwrap();
    let go_home = Callback::once(move |_| history.push(Route::Home));

    html! {
        <RequireLogin>
            <Layout>
                <Header variant={HeaderVariant::PageTitle}>
                    {translate("other", &translations)}
                </Header>
                <LogoutButton />
                <button onclick={go_home}>
                    {translate("go-home-page", &translations)}
                </button>
            </Layout>
        </RequireLogin>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let translations = use_translations();
    let login_state = use_context::<UseReducerHandle<LoginState>>().expect("no ctx found");

    let onclick = {
        let login_state = login_state.clone();
        Callback::from(move |_| login_state.dispatch(LoginAction::Logout))
    };

    html! {
        <button {onclick}>
            {translate("logout", &translations)}
        </button>
    }
}
