use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    class: Classes,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <main class={classes!("layout", props.class.clone())}>
            {for props.children.iter()}
        </main>
    }
}