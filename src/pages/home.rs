use crate::components::page::Page;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="SJA"/>
        <Page>
            <h1>Header</h1>
        </Page>
    }
}
