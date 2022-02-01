use yew::prelude::*;
use yew_router::prelude::*;

use shared_components::*;
use core::translations::{translate, Locale};
use crate::pages::login::{RequireLogin, LoginAction, LoginState};
use crate::routes::Route;
use crate::translations::{use_translations, TranslationAction, TranslationState};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let translations = use_translations();
    let translations_state = use_context::<UseReducerHandle<TranslationState>>().expect("no ctx found");
    let history = use_history().expect("no history found");
    let go_other = Callback::once(move |_| history.push(Route::Other));

    let locale_en = {
        let translations_state = translations_state.clone();
        Callback::from(move |_| translations_state.dispatch(TranslationAction::ChangeLocale(Locale::English)))
    };

    let locale_sv = {
        let translations_state = translations_state.clone();
        Callback::from(move |_| translations_state.dispatch(TranslationAction::ChangeLocale(Locale::Swedish)))
    };

    html! {
        <RequireLogin>
            <Layout>
                <Header variant={HeaderVariant::PageTitle}>
                    {translate("home", &translations)}
                </Header>
                <LogoutButton />
                <button onclick={go_other}>
                    {translate("go-other-page", &translations)}
                </button>
                <button onclick={locale_en}>
                    {translate("english", &translations)}
                </button>
                <button onclick={locale_sv}>
                    {translate("swedish", &translations)}
                </button>
            </Layout>
        </RequireLogin>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let translations = use_translations();
    let login_state = use_context::<UseReducerHandle<LoginState>>().expect("no ctx found");
    let onclick = Callback::from(move |_| login_state.dispatch(LoginAction::Logout));

    html! {
        <button {onclick}>
            {translate("logout", &translations)}
        </button>
    }
}