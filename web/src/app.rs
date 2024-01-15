use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[path="./routes/home.rs"]
mod home;
use home::Home;

#[path="./routes/docs.rs"]
mod docs;
use docs::Docs;

#[path="./routes/error_page.rs"]
mod error_page;
use error_page::Error_page;

#[path="./components/dialogs.rs"]
mod dialogs;
use dialogs::{
    Connection_dialog,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <>
            <main>
                <Title formatter=|text| format!("Anvel • {text}")/>
                <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <Meta charset="utf-8"/>
                <Meta name="description" content="Anvel is a productivity software that connects you to people in your network and enable file sharing and collaboration."/>
            </main>
            <Router>
                <Routes>
                    <Route path="/" view=  move || view! { <Home/> }/>
                    <Route path="/docs" view=Docs/>
                    // <Route path="/users/:id" view=UserProfile/>
                    <Route path="/*any" view=|| view! { <Error_page/> }/>
                </Routes>
            </Router>
            <Connection_dialog/>
        </>
    }
}