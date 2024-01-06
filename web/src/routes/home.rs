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

// const ORANGE_ICON:&str =r#"
//     color: orange
// "#;

#[component]
pub fn Home() -> impl IntoView {
    let window=window().expect("Failed to get Window");
    // let document=window.document().expect("Failed to get Document");
    let navigator=window.navigator();
    // window.alert_with_message(format!("Not online, {}",navigator.on_line()).as_str()).unwrap();

    if !navigator.on_line() {
        let closure: Closure<dyn FnMut()> = Closure::new(move|| {
            open_dialog();
        });
        
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref()
        ,300).unwrap();
        closure.forget();
    }


    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <Topnav/>
            <Sidenav/>

            <div class="ml-[200px] mt-[48px] mb-[22px] text-[#999999]">
                <div>
                    <div class="flex w-full h-[35px] bg-[#151515]">
                        <div class="bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                            <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder_open</span>
                            <p class="text-[#E5E5E5] text-[13px]">main.rs</p>
                            <span class="material-symbols-outlined md-16 w-[22px] ml-[3px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                        </div>

                        <div class="active:bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                            <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder_open</span>
                            <p class="text-[#E5E5E5] text-[13px]">telegram-desktop</p>
                            <span class="material-symbols-outlined md-16 w-[22px] p-[3px] ml-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                        </div>
                    </div>

                    <div class="w-full flex items-center h-[22px] text-[13px] text-[#999999] px-[16px]">
                        <p>src</p>
                        <span class="material-symbols-outlined md-16 w-[22px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">chevron_right</span>
                        <p>telegram-desktop</p>
                        <span class="material-symbols-outlined md-16 w-[22px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">chevron_right</span>
                        <p>main.rs</p>
                    </div>
                </div>

                <div>
                    <p>heykkkk</p>
                </div>
            </div>

            <Bottomnav/>
        </div>
    }
}