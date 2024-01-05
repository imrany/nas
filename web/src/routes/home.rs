use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;
use web_sys::{
 window,
};

#[path="../components/top_nav.rs"]
mod top_nav;
use top_nav::Topnav;

#[path="../components/side_nav.rs"]
mod side_nav;
use side_nav::Sidenav;

#[path="../components/bottom_nav.rs"]
mod bottom_nav;
use bottom_nav::Bottomnav;

#[path="../lib/functions.rs"]
mod functions;
use functions::{
    open_dialog,
};

#[path="../components/dialog.rs"]
mod dialog;
use dialog::Dialog;

// const ORANGE_ICON:&str =r#"
//     color: orange
// "#;

#[component]
pub fn Home() -> impl IntoView {
    let window=window().expect("Failed to get Window");
    let document=window.document().expect("Failed to get Document");
    let navigator=window.navigator();

    // open_dialog();
    let open=move||{
        let dialog_bg=document.get_element_by_id("dialog_bg").expect("Failed to get element by id 'dialog_bg'");
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().add_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap(); 
    };
    if !navigator.on_line() {
        // window.alert_with_message(format!("Not online, {}",navigator.on_line()).as_str()).unwrap();
        open();
        let closure: Closure<dyn FnMut()> = Closure::new(move|| {
            web_sys::console::log_1(&"Timeout".into());
        });
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref()
        ,500).unwrap();
    }


    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <Topnav/>
            <Sidenav/>

            <div class="ml-[200px] mt-[48px] mb-[22px] text-[#999999]">
                <p>heykkkk</p>
            </div>

            <Bottomnav/>
            <Dialog/>
        </div>
    }
}