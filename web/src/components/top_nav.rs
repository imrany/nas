use leptos::*;
use web_sys::{
    window,
    Event,
};
use wasm_bindgen::prelude::*;

#[component]
pub fn Topnav()-> impl IntoView{
    let window=window().unwrap();
    let document=window.document().unwrap();
    
    // Close the dropdown if the user clicks outside of it
    let document_ref=document.clone();
    let click_handler: Closure<dyn FnMut(_)> = Closure::new(move|e: Event| {
        let dropdowns=document_ref.get_elements_by_class_name("dropdown-content");
        web_sys::console::log_1(&e.target().into());

        for i in 0..dropdowns.clone().length(){
            let open_dropdown=dropdowns.get_with_index(i);
            if open_dropdown.clone().unwrap().class_list().contains("block") {
                open_dropdown.clone().unwrap().class_list().remove_1("block").unwrap();
            }
        }
    });
    window.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();
    
    let document_ref_0=document.clone();
    let open_share_dialog=move|_|{
        let dialog_bg=document_ref_0.get_element_by_id("dialog_bg").unwrap();
        dialog_bg.class_list().add_1("ease-in-out").unwrap();
        dialog_bg.class_list().add_1("block").unwrap();
        dialog_bg.class_list().add_1("duration-1000").unwrap();
        dialog_bg.class_list().add_1("delay-2000").unwrap();
    };

    let document_ref_1=document.clone();
    let show_dropdown_menu=move|_|{
        let dropdown_list=document_ref_1.get_element_by_id("dropdown_list").unwrap();
        dropdown_list.class_list().toggle("block").unwrap();
    };
    view!{
        <nav class="fixed bg-[#151515] px-[12px] top-0 left-0 right-0 z-10">
            <div class="font-semibold text-[13px] flex justify-between min-h-[48px] items-center text-white">
                <div style="position:relative; display:inline-block;">
                    <button on:click=show_dropdown_menu class="flex justify-center items-center py-[4px] px-[12px] cursor-pointer h-[24px]">
                        <p>Zippy</p>
                        <span class="material-symbols-outlined md-18 p-[3px]">expand_more</span>
                    </button>
                    <div id="dropdown_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="z-10 py-[4px] dropdown-content none absolute bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                        <div class="px-[12px] py-[8px] flex items-center">
                            <p>Documentation</p>
                        </div>
                        <div class="px-[12px] py-[8px] flex items-center">
                            <p>Version 1.0.2</p>
                        </div>
                        <div class="px-[12px] py-[8px] flex items-center">
                            <p>"Sign in"</p>
                        </div>
                    </div>
                </div>
                <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                    <p class="font-medium">Downloads/Telegram</p>
                </div>
                <div class="text-[#C2C2C2] flex min-w-[10%] justify-around">
                    <button 
                        on:click=open_share_dialog
                        class="hover:text-white active:text-white focus:text-white"
                    >
                        Share
                    </button>
                    <a href="/" class="hover:text-white active:text-white focus:text-white">Sign in</a>
                </div>
            </div>
        </nav>
    }
}