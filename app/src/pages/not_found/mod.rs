use yew::prelude::*;

use shared_components::*;
use core::translations::translate;
use crate::translations::use_translations;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let translations = use_translations();

    html! {
        <Layout>
            <Header variant={HeaderVariant::PageTitle}>
                {translate("not-found", &translations)}
            </Header>
        </Layout>
    }
}