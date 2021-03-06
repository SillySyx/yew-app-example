use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{
    home::*,
    other::*,
    not_found::*,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/other")]
    Other,
    
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <HomePage />
        },
        Route::Other => html! {
            <OtherPage />
        },
        Route::NotFound => html! { 
            <NotFoundPage />
        },
    }
}