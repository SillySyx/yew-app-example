use yew::prelude::*;

#[derive(PartialEq)]
pub enum HeaderVariant {
    Normal,
    Large,
    PageTitle,
}

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(HeaderVariant::Normal)]
    pub variant: HeaderVariant,
    #[prop_or_default]
    class: Classes,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let size = get_size_from_variant(&props.variant);

    html! {
        <@{format!("h{}", size)} class={classes!("header", props.class.clone())}>
            { for props.children.iter() }
        </@>
    }
}

fn get_size_from_variant(variant: &HeaderVariant) -> u8 {
    match variant {
        HeaderVariant::Normal => 3,
        HeaderVariant::Large => 2,
        HeaderVariant::PageTitle => 1,
    }
}