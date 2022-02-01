use yew_router::prelude::*;
use yew::prelude::*;

use crate::routes::{Route, switch};
use crate::translations::TranslationState;

#[function_component(App)]
pub fn app() -> Html {
    let translations_state = use_reducer(TranslationState::default);

    html! {
        <ContextProvider<UseReducerHandle<TranslationState>> context={translations_state.clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<UseReducerHandle<TranslationState>>>
    }
}