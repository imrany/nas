use leptos::*;
use leptos_meta::*;

#[component]
pub fn Docs() -> impl IntoView {
    // let navigate = leptos_router::use_navigate();
    // navigate("/somewhere", Default::default());
    view! {
        <Title text="Docs"/>
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">Docs page</h2>
            <a href="/">Back home</a>
        </div>
    }
}