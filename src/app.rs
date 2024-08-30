use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::home,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="de" />
        <Title text="SJA" />
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=home::Home ssr=SsrMode::Async />
                </Routes>
            </main>
        </Router>
    }
}
